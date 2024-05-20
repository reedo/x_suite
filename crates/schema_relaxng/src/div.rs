use crate::pattern::Pattern;
use crate::{DatatypeLibrary, Namespace};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Div {
    #[serde(rename = "@ns")]
    pub ns: Namespace,

    #[serde(rename = "@datatypeLibrary")]
    pub datatype_library: DatatypeLibrary,

    #[serde(rename = "$value")]
    pub content: Option<Vec<Pattern>>,
}
