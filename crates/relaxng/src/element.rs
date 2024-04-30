use serde::{Deserialize, Serialize};

use crate::pattern::Pattern;
use crate::{AnyName, DatatypeLibrary, NCName, Namespace};

#[derive(Debug, Deserialize, Serialize)]
pub struct Element {
    #[serde(rename = "@ns")]
    pub ns: Namespace,

    #[serde(rename = "@datatypeLibrary")]
    pub datatype_library: DatatypeLibrary,

    #[serde(rename = "@name")]
    pub name: Option<NCName>,

    #[serde(rename = "anyName")]
    pub any_name: Option<AnyName>,

    #[serde(rename = "$value")]
    pub content: Option<Vec<Pattern>>,
}
