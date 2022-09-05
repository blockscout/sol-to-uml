pub mod handlers;
mod metrics;
mod settings;
mod tracer;
pub mod types;

pub use self::{settings::Settings, tracer::init_logs};
use actix_web::{web, App, HttpServer};
use futures::future;
use handlers::{sol_to_storage, sol_to_uml};
use metrics::Metrics;
use tracing_actix_web::TracingLogger;

pub async fn run(settings: Settings) -> std::io::Result<()> {
    let socket_addr = settings.server.addr;
    let metrics_enabled = settings.metrics.enabled;
    let metrics_addr = settings.metrics.addr;
    let metrics_endpoint = settings.metrics.route.clone();

    tracing::info!("sol_to_uml server is starting at {}", socket_addr);
    let metrics = Metrics::new(metrics_endpoint);
    let server_future = {
        let middleware = metrics.middleware().clone();
        HttpServer::new(move || {
            App::new()
                .wrap(middleware.clone())
                .wrap(TracingLogger::default())
                .service(
                    web::scope("/solidity")
                        .route("/uml", web::post().to(sol_to_uml))
                        .route("/storage", web::post().to(sol_to_storage)),
                )
        })
        .bind(socket_addr)?
        .run()
    };
    let mut futures = vec![tokio::spawn(async move { server_future.await })];
    if metrics_enabled {
        futures.push(tokio::spawn(async move {
            metrics.run_server(metrics_addr).await
        }))
    }
    let (res, _, others) = future::select_all(futures).await;
    for future in others.into_iter() {
        future.abort()
    }
    res?
}
