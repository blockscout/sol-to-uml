use sol_to_uml::{run, Config};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let config = Config::parse().expect("Failed to parse config");
    run(config).await
}
