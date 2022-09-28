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
use visualizer::{OutputMask, ResponseFieldMask};

fn sources(sources: HashMap<String, String>) -> BTreeMap<PathBuf, String> {
    sources
        .into_iter()
        .map(|(path, content)| (PathBuf::from(path), content))
        .collect()
}

fn output_mask(mask: Option<FieldMask>) -> Result<OutputMask, anyhow::Error> {
    mask.map(|mask| {
        mask.paths
            .into_iter()
            .map(ResponseFieldMask::try_from)
            .collect::<Result<HashSet<_>, anyhow::Error>>()
            .map(OutputMask)
    })
    .unwrap_or_else(|| Ok(Default::default()))
}

impl TryFrom<VisualizeContractsRequest> for visualizer::VisualizeContractsRequest {
    type Error = anyhow::Error;

    fn try_from(request: VisualizeContractsRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            sources: sources(request.sources),
            output_mask: output_mask(request.output_mask)?,
        })
    }
}

impl TryFrom<VisualizeStorageRequest> for visualizer::VisualizeStorageRequest {
    type Error = anyhow::Error;

    fn try_from(request: VisualizeStorageRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            sources: sources(request.sources),
            file_path: PathBuf::from(request.file_name),
            contract_name: request.contract_name,
            output_mask: output_mask(request.output_mask)?,
        })
    }
}

impl From<visualizer::Response> for VisualizeResponse {
    fn from(response: visualizer::Response) -> Self {
        Self {
            png: response.png,
            svg: response.svg,
        }
    }
}
