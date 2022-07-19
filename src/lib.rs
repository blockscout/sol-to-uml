mod cli;
mod config;
mod types;

pub use crate::config::Config;
use actix_web::{
    error,
    web::{self, Json},
    App, Error, HttpServer,
};
use std::io::{Error as StdError, Write};
use tempfile::TempDir;
use tokio::{process::Command, task::JoinError};
use types::{SolToUmlRequest, SolToUmlResponse};

async fn sol_to_uml(data: Json<SolToUmlRequest>) -> Result<Json<SolToUmlResponse>, Error> {
    let data = data.into_inner();
    let contract_dir = TempDir::new()?;
    let contract_path = contract_dir.path();

    let join = data.sources.into_iter().map(|(name, content)| {
        let contract_path = contract_path.to_owned();
        tokio::task::spawn_blocking(move || {
            let file_path = contract_path.join(name);
            let prefix = file_path.parent();
            if let Some(prefix) = prefix {
                std::fs::create_dir_all(prefix)?;
            }
            let mut f = std::fs::File::create(file_path)?;
            f.write_all(content.as_bytes())?;
            Ok(())
        })
    });
    let results: Vec<Result<Result<_, StdError>, JoinError>> =
        futures::future::join_all(join).await;
    for result in results {
        result.map_err(error::ErrorBadRequest)??;
    }

    let uml_path = contract_path.join("result.svg");
    let status = Command::new("sol2uml")
        .arg(contract_path)
        .arg("-o")
        .arg(uml_path.as_path())
        .status()
        .await?;

    log::info!("process finished with: {}", status);

    if status.success() {
        let uml_diagram = tokio::fs::read_to_string(uml_path).await?;
        Ok(Json(SolToUmlResponse { uml_diagram }))
    } else {
        Err(error::ErrorBadRequest(""))
    }
}

pub async fn run(config: Config) -> std::io::Result<()> {
    let socket_addr = config.server.addr;

    log::info!("sol_to_uml server is starting at {}", socket_addr);
    HttpServer::new(move || {
        App::new().service(web::resource("/sol2uml").route(web::post().to(sol_to_uml)))
    })
    .bind(socket_addr)?
    .run()
    .await
}
