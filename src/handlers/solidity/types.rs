use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct SolToUmlRequest {
    pub sources: BTreeMap<PathBuf, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SolToUmlResponse {
    pub uml_diagram: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SolToStorageRequest {
    pub sources: BTreeMap<PathBuf, String>,
    pub main_contract: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SolToStorageResponse {
    pub storage: String,
}
