use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let service = Arc::new(visualizer_server::SolidityVisualizerService::default());
    let grpc_server = visualizer_server::run::grpc_server(service.clone(), 8051);
    let http_server = visualizer_server::run::http_server(service, 8050);
    futures::try_join!(grpc_server, http_server)?;
    Ok(())
}
