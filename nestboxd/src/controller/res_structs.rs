use bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NestboxResponse {
    pub uuid: String,
    pub created_at: String
}


pub trait MapDocument {
    fn map_doc(doc: Document) -> Self;
}

impl MapDocument for NestboxResponse {
    fn map_doc(doc: Document) -> Self {
        let mut uuid = String::from("");
        let mut created_at = String::from("");
        if let Some(b) = doc.get("uuid") {
            uuid = b.to_string().replace('"', "");
        }
        if let Some(b) = doc.get("created_at") {
            created_at = b.as_datetime().unwrap().to_string();
        }
        NestboxResponse { uuid, created_at}
    }
}
