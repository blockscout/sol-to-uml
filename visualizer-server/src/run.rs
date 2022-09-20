use std::sync::Arc;

use actix_web::{App, HttpServer};

use crate::solidity::{route_solidity_visualizer, SolidityVisualizerService};

use crate::proto::blockscout::visualizer::v1::solidity_visualizer_server::SolidityVisualizerServer;

pub async fn http_server(port: u16) -> Result<(), anyhow::Error> {
    let service = Arc::new(SolidityVisualizerService::default());

    let server = HttpServer::new(move || {
        App::new().configure(|config| route_solidity_visualizer(config, service.clone()))
    })
    .bind(("0.0.0.0", port))
    .unwrap_or_else(|_| panic!("failed to bind server on port {}", port));

    server.run().await?;
    Ok(())
}

pub async fn grpc_server(port: u16) -> Result<(), anyhow::Error> {
    let addr = ([0, 0, 0, 0], port).into();
    let visualizer = SolidityVisualizerService::default();

    let server = tonic::transport::Server::builder()
        .add_service(SolidityVisualizerServer::new(visualizer));

    server.serve(addr).await?;
    Ok(())
}
