use crate::pattern::Pattern;
use crate::{Combine, DatatypeLibrary, NCName, Namespace};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Define {
    #[serde(rename = "@ns")]
    pub ns: Namespace,

    #[serde(rename = "@datatypeLibrary")]
    pub datatype_library: DatatypeLibrary,

    #[serde(rename = "@combine")]
    pub combine: Option<Combine>,

    #[serde(rename = "@name")]
    pub name: NCName,

    #[serde(rename = "$value")]
    pub content: Option<Vec<Pattern>>,
}
