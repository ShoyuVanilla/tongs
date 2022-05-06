use crate::tracing::spawn_blocking_with_tracing;
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::SqlitePool;
use std::str::FromStr;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &SqlitePool,
) -> Result<Option<(Uuid, String)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT user_id, password_hash
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
    .context("Failed to query stored credentials.")?
    .map(|row| (row.user_id, row.password_hash));
    if let Some((user_id, password_hash)) = row {
        let user_id = Uuid::from_str(&user_id).context("Stored user_id is not a valid uuid")?;
        Ok(Some((user_id, password_hash)))
    } else {
        Ok(None)
    }
}

#[tracing::instrument(name = "Validate credentials", skip(username, password, pool))]
pub async fn validate_password(
    username: String,
    password: String,
    pool: &SqlitePool,
) -> Result<Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash =
        "$argon2id$v=19$m=15000,t=2,p=1$ZUNMV29jT0ExNDRiZ1JjWA$IvsewjropWn/1W+tjUJxKw".to_string();

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&username, pool).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || verify_password_hash(expected_password_hash, password))
        .await
        .context("Failed to spawn blocking task.")??;

    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown user."))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
    name = "Validate credentials",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(&expected_password_hash).unwrap();

    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}
