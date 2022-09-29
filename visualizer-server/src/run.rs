use std::{net::SocketAddr, sync::Arc};

use actix_web::{dev::Server, App, HttpServer};

use crate::{solidity::route_solidity_visualizer, Settings, SolidityVisualizerService};

use crate::proto::blockscout::visualizer::v1::solidity_visualizer_server::SolidityVisualizerServer;

fn http_server(service: Arc<SolidityVisualizerService>, addr: SocketAddr) -> Server {
    log::info!("starting http server on addr {}", addr);
    let server = HttpServer::new(move || {
        App::new().configure(|config| route_solidity_visualizer(config, service.clone()))
    })
    .bind(addr)
    .unwrap_or_else(|_| panic!("failed to bind server"));

    server.run()
}

async fn grpc_server(
    service: Arc<SolidityVisualizerService>,
    addr: SocketAddr,
) -> Result<(), anyhow::Error> {
    log::info!("starting grpc server on addr {}", addr);
    let server = tonic::transport::Server::builder()
        .add_service(SolidityVisualizerServer::from_arc(service));

    server.serve(addr).await?;
    Ok(())
}

pub async fn sol2uml(settings: Settings) -> Result<(), anyhow::Error> {
    let service = Arc::new(SolidityVisualizerService::default());

    let mut futures = vec![];

    if settings.server.http.enabled {
        let http_server = {
            let http_server_future = http_server(service.clone(), settings.server.http.addr);
            tokio::spawn(async move { http_server_future.await.map_err(anyhow::Error::msg) })
        };
        futures.push(http_server)
    }

    if settings.server.grpc.enabled {
        let grpc_server = {
            let service = service.clone();
            tokio::spawn(async move { grpc_server(service, settings.server.grpc.addr).await })
        };
        futures.push(grpc_server)
    }

    let (res, _, others) = futures::future::select_all(futures).await;
    for future in others.into_iter() {
        future.abort()
    }
    res?
}
