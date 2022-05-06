use actix_web::{web, HttpResponse};
use anyhow::Context;
use shared::{UserInfo, UserInfoWrapper};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::session::TypedSession;

#[tracing::instrument(name = "Get current user", skip(session, db_pool))]
pub async fn get_current_user(
    session: TypedSession,
    db_pool: web::Data<SqlitePool>,
) -> HttpResponse {
    if let Some(user_id) = session.get_user_id().unwrap() {
        let username = get_stored_user(user_id, &db_pool).await.unwrap().unwrap();
        HttpResponse::Ok().json(UserInfoWrapper {
            user: UserInfo { username },
        })
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[tracing::instrument(skip(user_id, pool))]
async fn get_stored_user(
    user_id: Uuid,
    pool: &SqlitePool,
) -> Result<Option<String>, anyhow::Error> {
    let user_id = user_id.to_string();
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .context("Failed to query stored user info")?
    .map(|row| row.username);
    Ok(row)
}
