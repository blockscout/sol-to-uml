use sol_to_uml::{run, Config};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = Config::parse().expect("Failed to parse config");
    run(config).await
}
