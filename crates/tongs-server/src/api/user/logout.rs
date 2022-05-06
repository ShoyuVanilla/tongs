use actix_web::HttpResponse;

use crate::session::TypedSession;

pub async fn post_logout(session: TypedSession) -> HttpResponse {
    session.log_out();
    HttpResponse::Ok().finish()
}
