use super::internal::{self, Error};
use crate::response::{Response, ResponseFieldMask};
use std::{
    collections::{BTreeMap, HashSet},
    ffi::OsStr,
    path::PathBuf,
};
use tempfile::TempDir;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualizeContractsRequest {
    sources: BTreeMap<PathBuf, String>,

    output_mask: HashSet<ResponseFieldMask>,
}

#[derive(Debug, Error)]
pub enum VisualizeContractsError {
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
    #[error("execution error: {0}")]
    Execution(String),
}

impl From<internal::Error> for VisualizeContractsError {
    fn from(error: Error) -> Self {
        match error {
            Error::Internal(err) => VisualizeContractsError::Internal(err),
            Error::Sol2Uml(err) => VisualizeContractsError::Execution(err),
        }
    }
}

pub async fn visualize_contracts(
    request: VisualizeContractsRequest,
) -> Result<Response, VisualizeContractsError> {
    let base_dir = TempDir::new().map_err(anyhow::Error::msg)?;
    let base_dir_path = base_dir.path();

    internal::save_files(base_dir_path, request.sources).await?;

    let output_file_path = base_dir_path.join("result.svg");
    let args: Vec<&dyn AsRef<OsStr>> = vec![
        &"class",
        &base_dir_path,
        &"--hideFilename",
        &"-o",
        &output_file_path,
    ];

    internal::sol2uml_call(args).await?;
    let output = tokio::fs::read(output_file_path)
        .await
        .map_err(anyhow::Error::msg)?;

    Ok(Response {
        svg: Some(output.into()),
        png: None,
    })
}
