use quick_xml::events::{BytesDecl, BytesText};

use crate::node::Document;

pub(crate) struct DocumentBuilder {
    _document: Document,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        DocumentBuilder {
            _document: Default::default(),
        }
    }

    pub fn build(self) -> Document {
        Document::default()
    }

    pub fn set_decl(&mut self, event: &BytesDecl) {
        self._document.decl = event.into();
    }

    pub fn set_doctype(&mut self, event: &BytesText) {
        self._document.doc_type = event.into();
    }
}
