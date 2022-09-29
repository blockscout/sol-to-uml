use visualizer_server::Settings;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let settings = Settings::new().expect("failed to read config");
    visualizer_server::run::sol2uml(settings).await
}
