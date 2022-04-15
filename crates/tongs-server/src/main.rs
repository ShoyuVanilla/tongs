use actix_web::{App, HttpServer};
use actix_web_lab::web::spa;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            spa()
                .index_file("./static/index.html")
                .static_resources_mount("/static")
                .static_resources_location("./static")
                .finish(),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
