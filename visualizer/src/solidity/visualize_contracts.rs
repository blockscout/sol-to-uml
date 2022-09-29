use super::internal::{self, Error, Sol2Uml};
use crate::response::{OutputMask, Response, ResponseFieldMask};
use std::{collections::BTreeMap, path::PathBuf};
use tempfile::TempDir;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualizeContractsRequest {
    pub sources: BTreeMap<PathBuf, String>,
    pub output_mask: OutputMask,
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

    let svg = if request.output_mask.contains(&ResponseFieldMask::Svg) {
        let output_file_path = base_dir_path.join("result.svg");
        Sol2Uml::new()
            .arg("class")
            .arg(&base_dir_path)
            .arg("--hideFilename")
            .args(["-f", "svg"])
            .arg("-o")
            .arg(&output_file_path)
            .call()
            .await?;

        let output = tokio::fs::read(output_file_path)
            .await
            .map_err(anyhow::Error::msg)?;
        Some(output)
    } else {
        None
    };
    let png = None;

    Ok(Response { svg, png })
}
