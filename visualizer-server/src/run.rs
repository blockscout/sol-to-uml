use crate::{
    health::HealthService,
    proto::blockscout::visualizer::v1::{
        health_actix::route_health, health_server::HealthServer,
        solidity_visualizer_server::SolidityVisualizerServer,
    },
    solidity::{route_solidity_visualizer, SolidityVisualizerService},
};
use actix_web::{dev::Server, App, HttpServer};
use std::sync::Arc;

pub fn http_server(
    visualizer: Arc<SolidityVisualizerService>,
    healthcheck: Arc<HealthService>,
    port: u16,
) -> Server {
    let server = HttpServer::new(move || {
        App::new()
            .configure(|config| route_solidity_visualizer(config, visualizer.clone()))
            .configure(|config| route_health(config, healthcheck.clone()))
    })
    .bind(("0.0.0.0", port))
    .unwrap_or_else(|_| panic!("failed to bind server on port {}", port));

    server.run()
}

pub async fn grpc_server(
    visualizer: Arc<SolidityVisualizerService>,
    healthcheck: Arc<HealthService>,
    port: u16,
) -> Result<(), anyhow::Error> {
    let addr = ([0, 0, 0, 0], port).into();
    let server = tonic::transport::Server::builder()
        .add_service(SolidityVisualizerServer::from_arc(visualizer))
        .add_service(HealthServer::from_arc(healthcheck));

    server.serve(addr).await?;
    Ok(())
}
