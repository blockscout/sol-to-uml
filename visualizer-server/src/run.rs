use std::sync::Arc;

use actix_web::{App, HttpServer};

use crate::solidity::{route_solidity_visualizer, SolidityVisualizerService};

use crate::proto::blockscout::visualizer::v1::solidity_visualizer_server::SolidityVisualizerServer;

pub async fn http_server(
    service: Arc<SolidityVisualizerService>,
    port: u16,
) -> Result<(), anyhow::Error> {
    let server = HttpServer::new(move || {
        App::new().configure(|config| route_solidity_visualizer(config, service.clone()))
    })
    .bind(("0.0.0.0", port))
    .unwrap_or_else(|_| panic!("failed to bind server on port {}", port));

    server.run().await?;
    Ok(())
}

pub async fn grpc_server(
    service: Arc<SolidityVisualizerService>,
    port: u16,
) -> Result<(), anyhow::Error> {
    let addr = ([0, 0, 0, 0], port).into();
    let server = tonic::transport::Server::builder()
        .add_service(SolidityVisualizerServer::from_arc(service));

    server.serve(addr).await?;
    Ok(())
}
