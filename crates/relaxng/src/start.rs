use serde::{Deserialize, Serialize};

use crate::pattern::Pattern;
use crate::{Combine, DatatypeLibrary, Namespace};

#[derive(Debug, Deserialize, Serialize)]
pub struct Start {
    #[serde(rename = "@ns")]
    pub ns: Namespace,

    #[serde(rename = "@datatypeLibrary")]
    pub datatype_library: DatatypeLibrary,

    #[serde(rename = "@combine")]
    pub combine: Option<Combine>,

    #[serde(rename = "$value")]
    pub content: Option<Vec<Pattern>>,
}
