use anyhow::Context;
use sol_to_uml::{init_logs, run, Settings};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new().context("failed to parse config")?;
    init_logs(settings.jaeger.clone());
    run(settings).await?;

    Ok(())
}
