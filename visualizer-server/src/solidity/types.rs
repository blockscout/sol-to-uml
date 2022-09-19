use crate::proto::{
    blockscout::visualizer::v1::{
        VisualizeContractsRequest, VisualizeResponse, VisualizeStorageRequest,
    },
    google::protobuf::FieldMask,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::PathBuf,
};
use visualizer::ResponseFieldMask;

fn sources(sources: HashMap<String, String>) -> BTreeMap<PathBuf, String> {
    sources
        .into_iter()
        .map(|(path, content)| (PathBuf::from(path), content))
        .collect()
}

fn output_mask(_mask: Option<FieldMask>) -> HashSet<ResponseFieldMask> {
    // TODO
    HashSet::new()
}

impl From<VisualizeContractsRequest> for visualizer::VisualizeContractsRequest {
    fn from(request: VisualizeContractsRequest) -> Self {
        Self {
            sources: sources(request.sources),
            output_mask: output_mask(request.output_mask),
        }
    }
}

impl From<VisualizeStorageRequest> for visualizer::VisualizeStorageRequest {
    fn from(request: VisualizeStorageRequest) -> Self {
        Self {
            sources: sources(request.sources),
            file_path: PathBuf::from(request.file_name),
            contract_name: request.contract_name,
            output_mask: output_mask(request.output_mask),
        }
    }
}

impl From<visualizer::Response> for VisualizeResponse {
    fn from(response: visualizer::Response) -> Self {
        Self {
            png: response.png.map(|b| b.to_vec()),
            svg: response.svg.map(|b| b.to_vec()),
        }
    }
}
