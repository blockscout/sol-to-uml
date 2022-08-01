use super::types::{SolToStorageRequest, SolToStorageResponse, SolToUmlRequest, SolToUmlResponse};
use actix_web::{error, web::Json, Error};
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    io::{Error as StdError, ErrorKind, Write},
    path::{Path, PathBuf},
};
use tempfile::TempDir;
use tokio::process::Command;

pub async fn sol_to_uml_handler(
    data: Json<SolToUmlRequest>,
) -> Result<Json<SolToUmlResponse>, Error> {
    let data = data.into_inner();
    let contract_dir = TempDir::new()?;
    let contract_path = contract_dir.path();

    save_files(contract_path, data.sources).await?;
    let uml_path = contract_path.join("result.svg");
    sol2uml_call(&[
        "class",
        contract_path
            .to_str()
            .ok_or_else(|| error::ErrorInternalServerError("Internal error"))?,
        "--hideFilename",
        "-o",
        uml_path
            .to_str()
            .ok_or_else(|| error::ErrorInternalServerError("Internal error"))?,
    ])
    .await?;
    let uml_diagram = tokio::fs::read_to_string(uml_path).await?;

    Ok(Json(SolToUmlResponse { uml_diagram }))
}

pub async fn sol_to_storage_handler(
    data: Json<SolToStorageRequest>,
) -> Result<Json<SolToStorageResponse>, Error> {
    let data = data.into_inner();
    let contract_dir = TempDir::new()?;
    let contract_path = contract_dir.path();

    save_files(contract_path, data.sources).await?;
    let storage_path = contract_path.join("result.svg");
    sol2uml_call(&[
        "storage",
        contract_path
            .to_str()
            .ok_or_else(|| error::ErrorInternalServerError("Internal error"))?,
        "-c",
        &data.main_contract[..],
        "-o",
        storage_path
            .to_str()
            .ok_or_else(|| error::ErrorInternalServerError("Internal error"))?,
    ])
    .await?;
    let storage = tokio::fs::read_to_string(storage_path).await?;

    Ok(Json(SolToStorageResponse { storage }))
}

async fn save_files(root: &Path, files: BTreeMap<PathBuf, String>) -> Result<(), Error> {
    let join = files.into_iter().map(|(name, content)| {
        let root = root.to_owned();
        tokio::task::spawn_blocking(move || -> Result<(), StdError> {
            if name.has_root() {
                return Err(StdError::new(
                    ErrorKind::Other,
                    "Error. All paths should be relative.",
                ));
            }

            let file_path = root.join(name);
            let prefix = file_path.parent();
            if let Some(prefix) = prefix {
                std::fs::create_dir_all(prefix)?;
            }
            let mut f = std::fs::File::create(file_path)?;
            f.write_all(content.as_bytes())?;
            Ok(())
        })
    });
    let results: Vec<_> = futures::future::join_all(join).await;
    for result in results {
        result
            .map_err(error::ErrorInternalServerError)?
            .map_err(error::ErrorBadRequest)?;
    }

    Ok(())
}

async fn sol2uml_call<I, S>(args: I) -> Result<(), Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new("sol2uml").args(args).output().await?;

    log::info!("process finished with output: {:?}", output);

    if output.status.success() && output.stderr.is_empty() {
        Ok(())
    } else {
        Err(error::ErrorBadRequest(
            std::str::from_utf8(&output.stderr)?.to_owned(),
        ))
    }
}
