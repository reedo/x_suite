use crate::choice::Choice;
use crate::element::Element;
use serde::{Deserialize, Serialize};

use crate::grammar::Grammar;
use crate::r#ref::Ref;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Pattern {
    Attribute,
    Choice(Choice),
    Data,
    Element(Element),
    Empty,
    ExternalRef,
    Grammar(Grammar),
    Group,
    List,
    Interleave,
    Mixed,
    NotAllowed,
    OneOrMore,
    Optional,
    ParentRef,
    Ref(Ref),
    Text,
    Value,
    ZeroOrMore,
}
