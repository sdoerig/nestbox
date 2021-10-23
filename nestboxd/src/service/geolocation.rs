use bson::doc;
use chrono::{Duration, Utc};
use uuid::Uuid;

use super::service_helper::{InsertResult};

use mongodb::{Collection};

#[derive(Clone)]
pub struct GeolocationService {
    collection: Collection,
}

impl GeolocationService {
    pub fn new(collection: Collection) -> Self {
        GeolocationService { collection }
    }

    pub async fn post_geolocation(&self, nestbox_uuid: &str, long: f32, lat: f32) -> InsertResult {
        // End current location before entering a new one.
        let now = Utc::now();
        match self
            .collection
            .update_many(
                doc! {"nestbox_uuid": nestbox_uuid, "until_date": {"$gt": &now}},
                doc! {"$set": {"until_date": &now}},
                None,
            )
            .await
        {
            Ok(_) => {},
            Err(_) => return InsertResult::TerminationError,
        };

        match self
            .collection
            .insert_one(
                doc! { "uuid" : Uuid::new_v4().to_string(),
                "nestbox_uuid" : nestbox_uuid, "from_date" : &now,
                "until_date" : Utc::now() + Duration::days(365000),
                "position" : { "type" : "point", "coordinates" : [ &long, &lat ] } },
                None,
            )
            .await
        {
            Ok(d) => InsertResult::Ok(d.inserted_id.to_string()),
            Err(_) => InsertResult::InsertError,
        }
        //db.geolocations.updateMany({"nestbox_uuid": "eb0c7048-2cda-471d-beb3-7777b7d54858", "until_date": {"$lt": new ISODate("2021-06-12T20:15:31Z")}}, {$set: {"until_date": new ISODate("2021-06-12T20:15:31Z")}})
    }
}
