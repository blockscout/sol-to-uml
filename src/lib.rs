mod cli;
mod config;
pub mod handlers;

pub use crate::config::Config;
use actix_web::{web, App, HttpServer};
use handlers::solidity::{sol_to_storage_handler, sol_to_uml_handler};

pub async fn run(config: Config) -> std::io::Result<()> {
    let socket_addr = config.server.addr;

    log::info!("sol_to_uml server is starting at {}", socket_addr);
    HttpServer::new(move || {
        App::new().service(
            web::scope("/solidity")
                .route("/uml", web::post().to(sol_to_uml_handler))
                .route("/storage", web::post().to(sol_to_storage_handler)),
        )
    })
    .bind(socket_addr)?
    .run()
    .await
}
