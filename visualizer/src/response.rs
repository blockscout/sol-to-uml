use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ResponseFieldMask {
    Svg,
    Png,
}

impl Display for ResponseFieldMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseFieldMask::Svg => f.write_str("svg"),
            ResponseFieldMask::Png => f.write_str("png"),
        }
    }
}

impl TryFrom<String> for ResponseFieldMask {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "svg" => Ok(ResponseFieldMask::Svg),
            "png" => Ok(ResponseFieldMask::Png),
            _ => Err(anyhow::anyhow!("invalid response filed mask: {}", value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OutputMask(pub HashSet<ResponseFieldMask>);

impl OutputMask {
    pub fn contains(&self, key: &ResponseFieldMask) -> bool {
        // empty output mask means that all fields must present
        self.0.is_empty() || self.0.contains(key)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Response {
    pub svg: Option<Vec<u8>>,
    pub png: Option<Vec<u8>>,
}
