use crate::session::TypedSession;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::FromRequest;
use actix_web_lab::middleware::Next;

// pub async fn reject_anonymous_users(
//     mut req: ServiceRequest,
//     next: Next<impl MessageBody>,
// ) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
//     let session = {
//         let (http_request, payload) = req.parts_mut();
//         TypedSession::from_request(http_request, payload).await
//     }?;
//
//     // match session.is_logged_in() {
//     //     true =>
//     //     false =>
//     // }
//     todo!()
// }
