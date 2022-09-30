use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let visualizer = Arc::new(visualizer_server::SolidityVisualizerService::default());
    let healthcheck = Arc::new(visualizer_server::HealthCheckService::default());

    let http_server = {
        let http_server_future =
            visualizer_server::run::http_server(visualizer.clone(), healthcheck.clone(), 8050);
        tokio::spawn(async move { http_server_future.await.map_err(anyhow::Error::msg) })
    };

    let grpc_server = {
        let service = visualizer.clone();
        tokio::spawn(async move {
            visualizer_server::run::grpc_server(service, healthcheck, 8051).await
        })
    };

    let futures = vec![http_server, grpc_server];
    let (res, _, others) = futures::future::select_all(futures).await;
    for future in others.into_iter() {
        future.abort()
    }
    res?
}
