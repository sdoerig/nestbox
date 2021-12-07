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
        let uuid = get_string_by_key(doc, "uuid");
        let created_at = get_date_time_by_key(doc, "created_at");
        let mandant_uuid = get_string_by_key(doc, "mandant_uuid");
        let mut mandant_name = String::new();
        let mut mandant_website = String::new();
        let images = get_vec_string_by_key(doc, "images");

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
        let username = get_string_by_key(doc, "username");
        let mut success = false;
        let session = get_string_by_key(doc, "session");

        if let Some(_b) = doc.get("success") {
            success = true;
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
        let uuid = get_string_by_key(doc, "uuid");
        let bird = get_string_by_key(doc, "bird");
        let bird_website = get_string_by_key(doc, "bird_website");

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
        let uuid = get_string_by_key(doc, "uuid");
        let nestbox_uuid = get_string_by_key(doc, "nestbox_uuid");
        let discovery_date = get_date_time_by_key(doc, "discovery_date");
        let user_uuid = get_string_by_key(doc, "user_uuid");
        // bird_uuid can be on top level or...
        let mut bird_uuid = get_string_by_key(doc, "bird_uuid");
        let mut bird = String::from("");

        // ... can result from a join over two collection and then it'll be found
        // in an own document - ugly I think of a better solution.
        if let Some(d) = get_doc_by_key(doc, "bird") {
            bird_uuid = get_string_by_key(d, "uuid");
            bird = get_string_by_key(d, "bird");
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

#[derive(Serialize, Deserialize)]
pub struct GeolocationResponse {
    //{
    //    "uuid" : Uuid::new_v4().to_string(),
    //    "nestbox_uuid" : nestbox_uuid,
    //    "from_date" : &now,
    //    "until_date" : DateTime::from( SystemTime::now() + Duration::new(31536000000, 0)),
    //    "position" : { "type" : "point", "coordinates" : [ &long, &lat ] } }
    pub uuid: String,
    pub nestbox_uuid: String,
    pub from_date: String,
    pub until_date: String,
    pub long: f64,
    pub lat: f64,
}

impl MapDocument for GeolocationResponse {
    fn map_doc(doc: &Document) -> Self {
        let uuid = get_string_by_key(doc, "uuid");
        let nestbox_uuid = get_string_by_key(doc, "nestbox_uuid");
        let from_date = get_date_time_by_key(doc, "from_date");
        let until_date = get_date_time_by_key(doc, "until_date");
        let mut long: f64 = 0.0;
        let mut lat: f64 = 0.0;
        if let Some(d) = get_doc_by_key(doc, "position") {
            let long_lat = get_vec_f64_by_key(d, "coordinates");
            if let Some(_long) = long_lat.get(0) {
                long = *_long;
            }
            if let Some(_lat) = long_lat.get(1) {
                lat = *_lat;
            }
        }
        GeolocationResponse {
            uuid,
            nestbox_uuid,
            from_date,
            until_date,
            long,
            lat,
        }
    }
}

fn get_string_by_key(doc: &Document, key: &str) -> String {
    if let Some(b) = doc.get(key) {
        b.to_string().replace('"', "")
    } else {
        String::from("")
    }
}

fn get_date_time_by_key(doc: &Document, key: &str) -> String {
    if let Some(b) = doc.get(key) {
        if let Some(dt) = b.as_datetime() {
            dt.to_string()
        } else {
            String::from("")
        }
    } else {
        String::from("")
    }
}

fn get_vec_string_by_key(doc: &Document, key: &str) -> Vec<String> {
    let mut vec_str: Vec<String> = Vec::new();
    if let Ok(v) = doc.get_array(key) {
        for i in v {
            vec_str.push(i.to_string().replace('"', ""));
        }
    }
    vec_str
}

fn get_vec_f64_by_key(doc: &Document, key: &str) -> Vec<f64> {
    let mut vec_str: Vec<f64> = Vec::new();
    if let Ok(v) = doc.get_array(key) {
        for i in v {
            if let Some(f) = i.as_f64() {
                vec_str.push(f);
            }
        }
    }
    vec_str
}

fn get_doc_by_key<'a>(doc: &'a Document, key: &str) -> Option<&'a Document> {
    if let Ok(b) = doc.get_array(key) {
        if let Some(t) = b.get(0) {
            return t.as_document();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use super::{
        BirdResponse, BreedResponse, GeolocationResponse, LoginResponse, MapDocument,
        NestboxResponse,
    };
    use mongodb::bson::DateTime;
    use mongodb::bson::{doc, Document};
    const UUID: &str = "0b5cec76-02ac-4c6e-933e-62ebfae3e337";
    const NESTBOX_UUID: &str = "6f25fd00-011a-462f-aa3d-6959e6809017";
    const BIRD_UUID: &str = "ebe661d6-77ba-4bd1-bae3-9e4e7eb880a6";
    const BIRD_NAME: &str = "bird_17";
    const DISCOVERY_DATE: &str = "2021-06-01 18:36:38.989 UTC";

    #[actix_rt::test]
    async fn test_breed_response_from_db() {
        let db_mock_breed_db_doc = doc! {
        "uuid": UUID,
        "nestbox_uuid":NESTBOX_UUID,
        "discovery_date": DateTime::from_millis(1622572598989),
        "bird":[{"uuid": BIRD_UUID,"bird": BIRD_NAME}]};
        let breed_response = BreedResponse::map_doc(&db_mock_breed_db_doc);
        assert!(
            breed_response.uuid == UUID,
            "DB response: Breed response uuid {} should be {}",
            breed_response.uuid,
            UUID
        );
        assert!(
            breed_response.bird_uuid == BIRD_UUID,
            "DB response: Breed response bird_uuid {} should be {}",
            breed_response.bird_uuid,
            BIRD_UUID
        );
        assert!(
            breed_response.nestbox_uuid == NESTBOX_UUID,
            "DB response: Breed response nestbox_uuid {} should be {}",
            breed_response.nestbox_uuid,
            NESTBOX_UUID
        );
        assert!(
            breed_response.bird == BIRD_NAME,
            "DB response: Breed response bird {} should be {}",
            breed_response.bird,
            BIRD_NAME
        );
        assert!(
            breed_response.discovery_date == DISCOVERY_DATE,
            "DB response: Breed response discovery_date {} should be {}",
            breed_response.discovery_date,
            DISCOVERY_DATE
        );
    }

    #[actix_rt::test]
    async fn test_breed_response_post_new_breed() {
        let db_mock_breed_post_doc = doc! {
        "uuid": UUID,
        "nestbox_uuid":NESTBOX_UUID,
        "discovery_date": {"$date":{"$numberLong":"1622572598989"}},
        "bird_uuid": BIRD_UUID};
        let breed_response = BreedResponse::map_doc(&db_mock_breed_post_doc);
        assert!(
            breed_response.uuid == UUID,
            "Post response: Breed response uuid {} should be {}",
            breed_response.uuid,
            UUID
        );
        assert!(
            breed_response.bird_uuid == BIRD_UUID,
            "Post response: Breed response bird_uuid {} should be {}",
            breed_response.bird_uuid,
            BIRD_UUID
        );
        assert!(
            breed_response.nestbox_uuid == NESTBOX_UUID,
            "Post response: Breed response nestbox_uuid {} should be {}",
            breed_response.nestbox_uuid,
            NESTBOX_UUID
        );
        assert!(
            breed_response.bird == *"",
            "DB response: Breed response bird {} should be {}",
            breed_response.bird,
            String::from("")
        );
    }

    #[actix_rt::test]
    async fn test_() {
        let geo_loc = doc! {
        "uuid" : UUID,
        "nestbox_uuid" : NESTBOX_UUID,
        "from_date" : DateTime::from( SystemTime::now() + Duration::new(31536000000, 0)),
        "until_date" : DateTime::from( SystemTime::now() + Duration::new(31536000000, 0)),
        "position" : { "type" : "point", "coordinates" : [ 8.567, 46.2345667 ] } };
    }
}
