use bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NestboxResponse {
    pub uuid: String,
}

pub trait MapDocument {
    fn map_doc(doc: Document) -> Self;
}




impl MapDocument for NestboxResponse {
    fn map_doc(doc: Document) -> Self {
        let mut uuid = String::from("");
        if let Some(b) = doc.get("uuid") {
            uuid = b.to_string().replace('"', "");
        }
        NestboxResponse { uuid }
    }
}
