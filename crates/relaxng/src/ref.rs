use serde::{Deserialize, Serialize};

use crate::{DatatypeLibrary, NCName, Namespace};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ref {
    #[serde(rename = "@ns")]
    pub ns: Namespace,

    #[serde(rename = "@datatypeLibrary")]
    pub datatype_library: DatatypeLibrary,

    #[serde(rename = "@name")]
    pub name: NCName,
}
