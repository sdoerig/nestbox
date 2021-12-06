use mongodb::bson::DateTime;
use mongodb::bson::{doc, Document};
//use chrono::{Duration, Utc};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

use super::service_helper::InsertResult;

use mongodb::{Collection, Database};

const GEOLOCATIONS: &str = "geolocations";

#[derive(Clone)]
pub struct GeolocationService {
    collection: Collection<Document>,
}

impl GeolocationService {
    pub fn new(db: &Database) -> Self {
        GeolocationService {
            collection: db.collection(GEOLOCATIONS),
        }
    }

    pub async fn post_geolocation(&self, nestbox_uuid: &str, long: f32, lat: f32) -> InsertResult {
        // End current location before entering a new one.
        let now = DateTime::now();
        match self
            .collection
            .update_many(
                doc! {"nestbox_uuid": nestbox_uuid, "until_date": {"$gt": &now}},
                doc! {"$set": {"until_date": &now}},
                None,
            )
            .await
        {
            Ok(_) => {}
            Err(_) => return InsertResult::TerminationError,
        };
        let geolocation = doc! {
        "uuid" : Uuid::new_v4().to_string(),
        "nestbox_uuid" : nestbox_uuid,
        "from_date" : &now,
        "until_date" : DateTime::from( SystemTime::now() + Duration::new(31536000000, 0)),
        "position" : { "type" : "point", "coordinates" : [ &long, &lat ] } };
        match self.collection.insert_one(&geolocation, None).await {
            Ok(_d) => InsertResult::Ok(geolocation),
            Err(_) => InsertResult::InsertError,
        }
        //db.geolocations.updateMany({"nestbox_uuid": "eb0c7048-2cda-471d-beb3-7777b7d54858", "until_date": {"$lt": new ISODate("2021-06-12T20:15:31Z")}}, {$set: {"until_date": new ISODate("2021-06-12T20:15:31Z")}})
    }
}
