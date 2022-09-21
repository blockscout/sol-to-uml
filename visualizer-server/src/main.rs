use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let service = Arc::new(visualizer_server::SolidityVisualizerService::default());
    let http_server = visualizer_server::run::http_server(service.clone(), 8050);
    let grpc_server = async move {
        tokio::task::spawn(visualizer_server::run::grpc_server(service, 8051));
        Ok(())
    };
    let (_, _) = futures::try_join!(http_server, grpc_server)?;
    Ok(())
}
