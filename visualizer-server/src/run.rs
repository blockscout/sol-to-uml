use crate::{
    health::HealthService,
    proto::blockscout::visualizer::v1::{
        health_actix::route_health, health_server::HealthServer,
        solidity_visualizer_server::SolidityVisualizerServer,
    },
    solidity::{route_solidity_visualizer, SolidityVisualizerService},
    Settings,
};
use actix_web::{dev::Server, App, HttpServer};
use std::{net::SocketAddr, sync::Arc};

pub fn http_server(
    visualizer: Arc<SolidityVisualizerService>,
    health: Arc<HealthService>,
    addr: SocketAddr,
) -> Server {
    let server = HttpServer::new(move || {
        App::new()
            .configure(|config| route_solidity_visualizer(config, visualizer.clone()))
            .configure(|config| route_health(config, health.clone()))
    })
    .bind(addr)
    .unwrap_or_else(|_| panic!("failed to bind server"));

    server.run()
}

pub async fn grpc_server(
    visualizer: Arc<SolidityVisualizerService>,
    health: Arc<HealthService>,
    addr: SocketAddr,
) -> Result<(), anyhow::Error> {
    log::info!("starting grpc server on addr {}", addr);
    let server = tonic::transport::Server::builder()
        .add_service(SolidityVisualizerServer::from_arc(visualizer))
        .add_service(HealthServer::from_arc(health));

    server.serve(addr).await?;
    Ok(())
}

pub async fn sol2uml(settings: Settings) -> Result<(), anyhow::Error> {
    let visualizer = Arc::new(SolidityVisualizerService::default());
    let health = Arc::new(HealthService::default());

    let mut futures = vec![];

    if settings.server.http.enabled {
        let http_server = {
            let http_server_future = http_server(
                visualizer.clone(),
                health.clone(),
                settings.server.http.addr,
            );
            tokio::spawn(async move { http_server_future.await.map_err(anyhow::Error::msg) })
        };
        futures.push(http_server)
    }

    if settings.server.grpc.enabled {
        let grpc_server = {
            let service = visualizer.clone();
            tokio::spawn(
                async move { grpc_server(service, health, settings.server.grpc.addr).await },
            )
        };
        futures.push(grpc_server)
    }

    let (res, _, others) = futures::future::select_all(futures).await;
    for future in others.into_iter() {
        future.abort()
    }
    res?
}
