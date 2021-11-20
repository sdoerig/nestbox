use std::iter::Map;

use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

pub trait MapDocument {
    fn map_doc(doc: &Document) -> Self;
}

#[derive(Serialize, Deserialize)]
pub struct NestboxResponse {
    pub uuid: String,
    pub created_at: String,
    pub images: Vec<String>,
    pub mandant_uuid: String,
    pub mandant_name: String,
    pub mandant_website: String,
}

impl MapDocument for NestboxResponse {
    /*
    {"public":true,
    "uuid":"1bec20fc-5416-4941-b7e4-e15aa26a5c7a",
    "mandant_uuid":"c7d880d5-c98d-40ee-bced-b5a0165420c0",
    "created_at":{"$date":"2021-06-01T18:36:38.418Z"},
    "mandant":[{"uuid":"c7d880d5-c98d-40ee-bced-b5a0165420c0","name":"BirdLife 0","website":"https://www.birdwatcher.ch"}]}
     */
    fn map_doc(doc: &Document) -> Self {
        let mut uuid = String::new();
        let mut created_at = String::new();
        let mut mandant_uuid = String::new();
        let mut mandant_name = String::new();
        let mut mandant_website = String::new();
        let mut images: Vec<String> = Vec::new();
        if let Some(b) = doc.get("uuid") {
            uuid = b.to_string().replace('"', "");
        }
        if let Some(b) = doc.get("created_at") {
            created_at = b.as_datetime().unwrap().to_string();
        }
        if let Ok(v) = doc.get_array("images") {
            for i in v {
                images.push(i.to_string().replace('"', ""));
            }
        }
        if let Some(b) = doc.get("mandant_uuid") {
            mandant_uuid = b.to_string().replace('"', "");
        }
        if let Ok(v) = doc.get_array("mandant") {
            if let Some(t) = v.get(0) {
                if let Some(d) = t.as_document() {
                    if let Some(val) = d.get("name") {
                        mandant_name = val.to_string().replace('"', "");
                    }
                    if let Some(val) = d.get("website") {
                        mandant_website = val.to_string().replace('"', "");
                    }
                }
            }
        }
        NestboxResponse {
            uuid,
            created_at,
            images,
            mandant_uuid,
            mandant_name,
            mandant_website,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub username: String,
    pub success: bool,
    pub session: String,
}

impl MapDocument for LoginResponse {
    /*
    {"username":"fg_199","success":true,"session":"28704470-0908-408e-938f-64dd2b7578b9"}
     */
    fn map_doc(doc: &Document) -> Self {
        let mut username = String::new();
        let mut success = false;
        let mut session = String::new();
        if let Some(b) = doc.get("username") {
            username = b.to_string().replace('"', "");
        }
        if let Some(b) = doc.get("success") {
            success = true;
        }
        if let Some(b) = doc.get("session") {
            session = b.to_string().replace('"', "");
        }
        LoginResponse {
            username,
            success,
            session,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BirdResponse {
    //"uuid":"decd3296-0d22-427a-b92c-51c0ac2ae23a","bird":"bird_0"
    pub uuid: String,
    pub bird: String,
    pub bird_website: String,
}

impl MapDocument for BirdResponse {
    fn map_doc(doc: &Document) -> Self {
        let mut uuid = String::new();
        let mut bird = String::new();
        let mut bird_website = String::new();
        if let Some(b) = doc.get("uuid") {
            uuid = b.to_string().replace('"', "");
        }
        if let Some(b) = doc.get("bird") {
            bird = b.to_string().replace('"', "")
        }
        if let Some(b) = doc.get("bird_website") {
            bird_website = b.to_string().replace('"', "");
        }
        BirdResponse {
            uuid,
            bird,
            bird_website,
        }
    }
}
