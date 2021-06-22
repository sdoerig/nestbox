use bson::doc;
use chrono::{Utc, Duration};
use uuid::Uuid;

use super::service_helper as sa;
use crate::controller::{
    req_structs::BirdReq,
    utilities::{DocumentResponse, PagingQuery},
};
use crate::controller::{req_structs::NestboxReq, utilities::SessionObject};
use mongodb::{error::Error, Collection};

#[derive(Clone)]
pub struct GeolocationService {
    collection: Collection,
}

impl GeolocationService {
    pub fn new(collection: Collection) -> Self {
        GeolocationService { collection }
    }

    pub async fn post_geolocation(&self, nestbox_uuid: &str, long: f32, lat: f32) {
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
            Ok(_) => todo!(),
            Err(_) => todo!(),
        };
        match self
            .collection
            .insert_one(
                doc! { "uuid" : "3490e91f-8d0a-4a7e-94a0-ccd17d9832ab",
                "nestbox_uuid" : nestbox_uuid, "from_date" : &now,
                "until_date" : Utc::now() + Duration::days(365000),
                "position" : { "type" : "point", "coordinates" : [ &long, &lat ] } },
                None,
            )
            .await
        {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
        //db.geolocations.updateMany({"nestbox_uuid": "eb0c7048-2cda-471d-beb3-7777b7d54858", "until_date": {"$lt": new ISODate("2021-06-12T20:15:31Z")}}, {$set: {"until_date": new ISODate("2021-06-12T20:15:31Z")}})
    }
}
