use std::collections::BTreeMap;
use std::str::from_utf8;

use quick_xml::events::BytesDecl;

pub struct DocDecl(pub String);

impl Default for DocDecl {
    fn default() -> Self {
        DocDecl("".to_owned())
    }
}

impl From<&BytesDecl<'_>> for DocDecl {
    fn from(value: &BytesDecl<'_>) -> Self {
        let str = from_utf8(value).unwrap_or("");
        DocDecl(str.to_owned())
    }
}

pub struct DocType(pub String);

impl Default for DocType {
    fn default() -> Self {
        DocType("".to_owned())
    }
}

impl From<&BytesDecl<'_>> for DocType {
    fn from(value: &BytesDecl<'_>) -> Self {
        let str = from_utf8(value).unwrap_or("");
        DocType(str.to_owned())
    }
}

pub struct Document {
    pub decl: DocDecl,
    pub doc_type: DocType,
    pub nodes: BTreeMap<NodeId, Node>, // Would HashMap be better?
    pub root: NodeId,
}

impl Default for Document {
    fn default() -> Self {
        Document {
            decl: Default::default(),
            doc_type: Default::default(),
            nodes: Default::default(),
            root: Default::default(),
        }
    }
}

pub type NodeId = usize;

pub enum Node {
    CData(CData),
    Comment(Comment),
    Element(Element),
    ProcessingInstruction(ProcessingInstruction),
    Text(Text),
}

pub struct CData {
    pub parent: Option<NodeId>,
    pub data: String,
}

pub struct Comment {
    pub parent: Option<NodeId>,
    pub data: String,
}

pub struct Element {
    pub parent: Option<NodeId>,
    pub attributes: Vec<Attribute>,
    pub children: Vec<NodeId>,
}

pub struct ProcessingInstruction {
    pub parent: Option<NodeId>,
    pub data: String,
}

pub struct Text {
    pub parent: Option<NodeId>,
    pub data: String,
}

pub struct Attribute {
    pub local_name: String,
    pub value: String,
}
