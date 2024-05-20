use quick_xml::events::Event;
use quick_xml::Reader;

pub use error::{Error, Result};

use crate::builder::DocumentBuilder;
use crate::node::Document;

mod builder;
mod error;
pub mod node;
pub mod document;

/// TODO
fn deserialize_to_document(xml: &str) -> Result<Document> {
    let mut reader = Reader::from_str(xml);
    let mut builder = DocumentBuilder::new();

    loop {
        match reader.read_event() {
            Err(e) => {
                return Err(Error::QuickXml(e));
            }

            Ok(Event::Eof) => {
                return Ok(builder.build());
            }

            Ok(Event::Decl(e)) => {
                builder.set_decl(&e);
                continue;
            }

            Ok(Event::DocType(e)) => {
                builder.set_doctype(&e);
                continue;
            }

            Ok(Event::Comment(_e)) => {
                continue;
            }

            Ok(Event::PI(_e)) => {
                continue;
            }

            Ok(Event::Text(_e)) => {
                continue;
            }

            Ok(Event::CData(_e)) => {
                continue;
            }

            Ok(Event::Start(_e)) | Ok(Event::Empty(_e)) => {
                continue;
            }

            Ok(Event::End(_e)) => {
                continue;
            }
        }
    }
}
