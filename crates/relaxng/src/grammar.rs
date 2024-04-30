use serde::{Deserialize, Serialize};

use crate::define::Define;
use crate::include::Include;
use crate::start::Start;
use crate::{DatatypeLibrary, Namespace};

#[derive(Debug, Deserialize, Serialize)]
pub struct Grammar {
    #[serde(rename = "@ns")]
    pub ns: Namespace,

    #[serde(rename = "@datatypeLibrary")]
    pub datatype_library: DatatypeLibrary,

    #[serde(rename = "$value")]
    pub content: Option<Vec<GrammarContent>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GrammarContent {
    Define(Define),
    Include(Include),
    Start(Start),
}
