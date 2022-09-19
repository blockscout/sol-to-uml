use bytes::Bytes;
use std::fmt::{Display, Formatter};

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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Response {
    pub svg: Option<Bytes>,
    pub png: Option<Bytes>,
}
