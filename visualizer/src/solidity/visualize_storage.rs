use super::internal::{self, Error};
use crate::response::{Response, ResponseFieldMask};
use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
};
use tempfile::TempDir;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualizeStorageRequest {
    pub sources: BTreeMap<PathBuf, String>,
    pub file_path: PathBuf,
    pub contract_name: String,
    pub output_mask: HashSet<ResponseFieldMask>,
}

#[derive(Debug, Error)]
pub enum VisualizeStorageError {
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
    #[error("file path should contain file name")]
    InvalidFileName,
    #[error("execution error: {0}")]
    Execution(String),
}

impl From<internal::Error> for VisualizeStorageError {
    fn from(error: Error) -> Self {
        match error {
            Error::Internal(err) => VisualizeStorageError::Internal(err),
            Error::Sol2Uml(err) => VisualizeStorageError::Execution(err),
        }
    }
}

pub async fn visualize_storage(
    request: VisualizeStorageRequest,
) -> Result<Response, VisualizeStorageError> {
    let base_dir = TempDir::new().map_err(anyhow::Error::msg)?;
    let base_dir_path = base_dir.path();

    let file_name = request
        .file_path
        .file_name()
        .ok_or(VisualizeStorageError::InvalidFileName)?;
    internal::save_files(base_dir_path, request.sources).await?;

    let output_file_path = base_dir_path.join("result.svg");
    let args: Vec<String> = vec![
        "storage".to_string(),
        base_dir_path.to_string_lossy().to_string(),
        "-c".to_string(),
        request.contract_name,
        "-cf".to_string(),
        file_name.to_string_lossy().to_string(),
        "-o".to_string(),
        output_file_path.to_string_lossy().to_string(),
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
