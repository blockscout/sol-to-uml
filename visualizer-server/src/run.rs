use std::sync::Arc;

use actix_web::{App, HttpServer};

use crate::solidity::{route_solidity_visualizer, SolidityVisualizerService};

pub async fn http_server(port: u16) -> Result<(), std::io::Error> {
    let service = Arc::new(SolidityVisualizerService::default());

    let server = HttpServer::new(move || {
        App::new().configure(|config| route_solidity_visualizer(config, service.clone()))
    })
    .bind(("0.0.0.0", port))
    .expect(&format!("failed to bind server on port {}", port));
    server.run().await
}
