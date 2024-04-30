use serde::{Deserialize, Serialize};

use crate::define::Define;
use crate::div::Div;
use crate::start::Start;

#[derive(Debug, Deserialize, Serialize)]
pub struct Include {
    #[serde(rename = "$value")]
    pub content: Option<Vec<IncludeContent>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum IncludeContent {
    Define(Define),
    Div(Div),
    Start(Start),
}
