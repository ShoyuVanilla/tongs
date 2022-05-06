use std::net::TcpListener;

use actix_session::SessionMiddleware;
use actix_web::{cookie::Key, dev::Server, web, App, HttpServer};
use actix_web_lab::web::spa;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, SqlitePool};
use tracing_actix_web::TracingLogger;

use crate::api::{
    post_login,
    user::{get_current_user, post_logout},
};

use crate::session::InMemorySessionStore;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build() -> anyhow::Result<Self> {
        let connection_pool = get_connection_pool();
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        let server = run(listener, connection_pool).await?;
        Ok(Self { port: 8080, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn get_connection_pool() -> SqlitePool {
    let mut options = SqliteConnectOptions::new().filename("dev.db");
    options.log_statements(tracing::log::LevelFilter::Trace);
    SqlitePoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(options)
}

async fn run(listener: TcpListener, db_pool: SqlitePool) -> anyhow::Result<Server> {
    let db_pool = web::Data::new(db_pool);
    let secret_key = Key::generate();
    let session_store = InMemorySessionStore::new();

    let server = HttpServer::new(move || {
        App::new() //.service(
            //web::scope("/tongs")
            .wrap(SessionMiddleware::new(
                session_store.clone(),
                secret_key.clone(),
            ))
            .wrap(TracingLogger::default())
            .service(
                web::scope("/api")
                    .route("/login", web::post().to(post_login))
                    .route("/user/logout", web::post().to(post_logout))
                    .route("/user/current", web::get().to(get_current_user)),
            )
            .service(
                spa()
                    .index_file("./static/index.html")
                    .static_resources_mount("/static")
                    .static_resources_location("./static")
                    .finish(),
            )
            .app_data(db_pool.clone()) //,
                                       // )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
