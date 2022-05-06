use actix_web::error::InternalError;
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use tracing::field::display;

use shared::{LoginInfoWrapper, UserInfo, UserInfoWrapper};

use crate::auth::password::{validate_password, AuthError};
use crate::session::TypedSession;

#[tracing::instrument(
    skip(session, db_pool, login_info),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn post_login(
    session: TypedSession,
    login_info: web::Json<LoginInfoWrapper>,
    db_pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, InternalError<AuthError>> {
    tracing::Span::current().record(
        "username",
        &tracing::field::display(&login_info.user.username),
    );
    match validate_password(
        login_info.user.username.clone(),
        login_info.user.password.clone(),
        &db_pool,
    )
    .await
    {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            session.insert_user_id(user_id);
            let user_info = UserInfo {
                username: login_info.user.username.clone(),
            };
            let resp = HttpResponse::Ok().json(UserInfoWrapper { user: user_info });
            Ok(resp)
        }
        Err(e) => {
            let resp = HttpResponse::BadRequest().finish();
            Err(InternalError::from_response(e, resp))
        }
    }
}
