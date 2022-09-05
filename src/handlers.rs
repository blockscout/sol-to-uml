use crate::{
    metrics,
    types::{SolToStorageRequest, SolToStorageResponse, SolToUmlRequest, SolToUmlResponse},
};
use actix_web::{error, web::Json, Error};
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    io::{Error as StdError, ErrorKind, Write},
    path::{Path, PathBuf},
};
use tempfile::TempDir;
use tokio::process::Command;
use tracing::instrument;

#[instrument(level = "debug")]
pub async fn sol_to_uml(data: Json<SolToUmlRequest>) -> Result<Json<SolToUmlResponse>, Error> {
    tracing::info!("UML request received, processing begins.");
    let data = data.into_inner();

    let response = sol_to_uml_handler(data).await;
    metrics::count_sol2uml_request(response.is_ok(), "uml");

    Ok(Json(response?))
}

async fn sol_to_uml_handler(data: SolToUmlRequest) -> Result<SolToUmlResponse, Error> {
    let contract_dir = TempDir::new()?;
    let contract_path = contract_dir.path();

    save_files(contract_path, data.sources).await?;
    let uml_path = contract_path.join("result.svg");
    let args: Vec<&dyn AsRef<OsStr>> = vec![
        &"class",
        &contract_path,
        &"--hideFilename",
        &"-o",
        &uml_path,
    ];
    sol2uml_call(args, "uml").await?;
    let uml_diagram = tokio::fs::read_to_string(uml_path).await?;
    tracing::info!("UML successfully created.");

    Ok(SolToUmlResponse { uml_diagram })
}

#[instrument(level = "debug")]
pub async fn sol_to_storage(
    data: Json<SolToStorageRequest>,
) -> Result<Json<SolToStorageResponse>, Error> {
    tracing::info!("Storage request received, processing begins.");
    let data = data.into_inner();

    let response = sol_to_storage_handler(data).await;
    metrics::count_sol2uml_request(response.is_ok(), "storage");

    Ok(Json(response?))
}

async fn sol_to_storage_handler(data: SolToStorageRequest) -> Result<SolToStorageResponse, Error> {
    let contract_dir = TempDir::new()?;
    let contract_path = contract_dir.path();

    tracing::info!("Storage request received, processing begins.");
    let main_contract_filename = data.main_contract_filename.file_name().ok_or_else(|| {
        error::ErrorBadRequest("Error. Main contract filename should contain filename.")
    })?;

    save_files(contract_path, data.sources).await?;
    let storage_path = contract_path.join("result.svg");
    let args: Vec<&dyn AsRef<OsStr>> = vec![
        &"storage",
        &contract_path,
        &"-c",
        &data.main_contract,
        &"-cf",
        &main_contract_filename,
        &"-o",
        &storage_path,
    ];

    sol2uml_call(args, "storage").await?;
    let storage = tokio::fs::read_to_string(storage_path).await?;
    tracing::info!("Storage successfully created.");

    Ok(SolToStorageResponse { storage })
}

#[instrument(level = "debug")]
async fn save_files(root: &Path, files: BTreeMap<PathBuf, String>) -> Result<(), Error> {
    let _timer = metrics::SAVEFILES_TIME.start_timer();
    let join = files.into_iter().map(|(name, content)| {
        let root = root.to_owned();
        tokio::task::spawn_blocking(move || -> Result<(), StdError> {
            if name.has_root() {
                tracing::error!("File path wasn`t relative {:?}.", name);
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
    tracing::debug!("Files saved successfully.");

    Ok(())
}

#[instrument(skip(args), level = "debug")]
async fn sol2uml_call<'a, I>(args: I, request_type: &str) -> Result<(), Error>
where
    I: IntoIterator<Item = &'a dyn AsRef<OsStr>>,
{
    let _timer = metrics::SOL2UML_RUN_TIME
        .with_label_values(&[request_type])
        .start_timer();
    let output = Command::new("cmd")
        .arg("/C")
        .arg("sol2uml")
        .args(args)
        .output()
        .await?;

    tracing::debug!("sol2uml process finished with output: {:?}", output);

    if output.status.success() && output.stderr.is_empty() {
        Ok(())
    } else {
        let e = std::str::from_utf8(&output.stderr)?;
        tracing::error!("sol2uml run failed: {}", e);
        Err(error::ErrorBadRequest(e.to_owned()))
    }
}
