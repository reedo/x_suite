use serde::{Deserialize, Serialize};

mod choice;
mod define;
mod div;
mod element;
mod grammar;
mod include;
mod pattern;
mod r#ref;
mod start;

pub type AnyURI = String;
pub type DatatypeLibrary = Option<AnyURI>;
pub type Namespace = Option<String>;
pub type NCName = String;
pub type QName = String;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "anyName")]
pub struct AnyName {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Combine {
    Choice,
    Interleave,
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use quick_xml::de::from_str;

    use crate::pattern::Pattern;

    #[test]
    fn it_works() -> Result<(), anyhow::Error> {
        let data = read_to_string("resources/relaxng.rng")?;
        let structure: Pattern = from_str(&data)?;

        dbg!(structure);

        Ok(())
    }
}
