use tongs_server::application::Application;
use tongs_server::tracing::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("tongs".to_string(), "info".to_string(), std::io::stdout);
    init_subscriber(subscriber);

    let application = Application::build().await?;
    application.run().await?;
    Ok(())
}
