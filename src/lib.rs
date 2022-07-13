mod cli;
mod config;
mod types;

pub use crate::config::Config;
use actix_web::{
    error,
    web::{self, Json},
    App, Error, HttpServer,
};
use tempfile::TempDir;
use tokio::{io::AsyncWriteExt, process::Command};
use types::{SolToUmlRequest, SolToUmlResponse};

async fn sol_to_uml(data: Json<SolToUmlRequest>) -> Result<Json<SolToUmlResponse>, Error> {
    let data = data.into_inner();
    let contract_dir = TempDir::new()?;
    let contract_path = contract_dir.path();

    for (name, content) in data.sources {
        let file_path = contract_path.join(name);
        let prefix = file_path.parent();
        if let Some(prefix) = prefix {
            tokio::fs::create_dir_all(prefix).await?;
        }

        let mut f = tokio::fs::File::create(file_path).await?;
        f.write_all(content.as_bytes()).await?;
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

    log::info!("Sol-to-uml server is starting at {}", socket_addr);
    HttpServer::new(move || {
        App::new().service(web::resource("/sol2uml").route(web::post().to(sol_to_uml)))
    })
    .bind(socket_addr)?
    .run()
    .await
}
