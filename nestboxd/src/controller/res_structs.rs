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
        if let Some(_b) = doc.get("success") {
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

#[derive(Serialize, Deserialize)]
pub struct BreedResponse {
    //{"uuid":"0b5cec76-02ac-4c6e-933e-62ebfae3e337",
    // "nestbox_uuid":"6f25fd00-011a-462f-aa3d-6959e6809017",
    // "discovery_date":{"$date":{"$numberLong":"1622572598989"}},
    // "bird":[{"uuid":"ebe661d6-77ba-4bd1-bae3-9e4e7eb880a6","bird":"bird_17"}]}
    pub uuid: String,
    pub nestbox_uuid: String,
    pub discovery_date: String,
    pub user_uuid: String,
    pub bird_uuid: String,
    pub bird: String,
}

impl MapDocument for BreedResponse {
    fn map_doc(doc: &Document) -> Self {
        let mut uuid = String::from("");
        let mut nestbox_uuid = String::from("");
        let mut discovery_date = String::from("");
        let mut user_uuid = String::from("");
        let mut bird_uuid = String::from("");
        let mut bird = String::from("");

        if let Some(b) = doc.get("uuid") {
            uuid = b.to_string().replace('"', "");
        }
        if let Some(b) = doc.get("nestbox_uuid") {
            nestbox_uuid = b.to_string().replace('"', "");
        }
        if let Some(b) = doc.get("user_uuid") {
            user_uuid = b.to_string().replace('"', "");
        }
        if let Some(b) = doc.get("discovery_date") {
            if let Some(dt) = b.as_datetime() {
                discovery_date = dt.to_string();
            }
        }
        if let Some(b) = doc.get("bird_uuid") {
            bird_uuid = b.to_string().replace('"', "");
        }
        if let Ok(b) = doc.get_array("bird") {
            if let Some(t) = b.get(0) {
                if let Some(d) = t.as_document() {
                    if let Some(bson_bird_uuid) = d.get("uuid") {
                        bird_uuid = bson_bird_uuid.to_string().replace('"', "");
                    }
                    if let Some(bson_bird) = d.get("bird") {
                        bird = bson_bird.to_string().replace('"', "");
                    }
                }
            }
        }

        BreedResponse {
            uuid,
            nestbox_uuid,
            discovery_date,
            user_uuid,
            bird_uuid,
            bird,
        }
    }
}
