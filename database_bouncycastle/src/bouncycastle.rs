//! # Database Bouncy Castle
//! As the name indicated, this programm is just for playing. Its main purpose is
//! to give my a feeling of the data model and stock the database the rest
//! will hopefully develop on with a reasonable amount of records.
//! This because I do feel unconfortable if developing something on
//! an empty database, because real life does not take place on an empty database.
//!
//!
use chrono::prelude::*;
use mongodb::bson::doc;
use mongodb::sync::Client;
use rand::Rng;
use sha3::{Digest, Sha3_256};

use uuid::Uuid;
mod collector;
use collector::{Collector, CollectorState};


const COL_NESTBOXES: &str = "nestboxes";
const COL_MANDANTS: &str = "mandants";
const COL_BREEDS: &str = "breeds";
const COL_USERS: &str = "users";
const COL_GEOLOCATIONS: &str = "geolocations";
const COL_BIRDS: &str = "birds";

pub fn populate_db(db_uri: &str, db_name: &str, records_to_insert: i32) -> mongodb::error::Result<()> {
    let client = Client::with_uri_str(db_uri)?;

    let database = client.database(db_name);
    let mut nestboxes_collector = Collector::new(database.collection(COL_NESTBOXES));
    let mut mandant_collector = Collector::new(database.collection(COL_MANDANTS));
    let mut breeds_collector = Collector::new(database.collection(COL_BREEDS));
    let mut users_collector = Collector::new(database.collection(COL_USERS));
    let mut geolocations_collector = Collector::new(database.collection(COL_GEOLOCATIONS));
    let mut birds_collector = Collector::new(database.collection(COL_BIRDS));
    mandant_collector.append_doc(doc! { "uuid": Uuid::new_v4().to_string(),
    "name": "BirdLife",
    "website": "https://www.birdwatcher.ch",
    "email": "bird@iseeyou.ch"});
    mandant_collector.flush();
    //let mut mandant_object = mandant_collector.result.get(&0).unwrap();
    let mut mandant_uuid = mandant_collector.uuids.get(0).unwrap().clone();
    gen_birds_for_mandant(&mut birds_collector, &mandant_uuid);
    for i in 0..records_to_insert as usize {
        let (user_password_salt, password_hash) = get_password_and_salt();
        users_collector.append_doc(doc! {
        "mandant_uuid": &mandant_uuid,
        "username": format!("fg_{}", i),
        "uuid": Uuid::new_v4().to_string(),
        "lastname": "Gucker",
        "firstname":"Fritz",
        "email": format!("email_{}@birdwatch.ch", i),
        "password_hash": password_hash,
        "salt": user_password_salt.to_string()});
        let nestbox_flushed = match nestboxes_collector.append_doc(
            doc! {"public": true, "uuid": Uuid::new_v4().to_string(), "mandant_uuid": &mandant_uuid, "created_at": Utc::now()},
        ) {
            CollectorState::Flushed => true,
            CollectorState::Accumulating => false,
        };
        if nestbox_flushed {
            generate_nestboxes_additionals(
                &users_collector,
                &nestboxes_collector,
                &birds_collector,
                &mut geolocations_collector,
                &mut breeds_collector,
            );
        }
        if i % 100 == 0 {
            mandant_collector.append_doc(
                doc!{ "uuid": Uuid::new_v4().to_string(), 
                    "name": format!("BirdLife {}", i),  "website": "https://www.birdwatcher.ch", "email": "bird@iseeyou.ch"});
            mandant_collector.flush();
            mandant_uuid = mandant_collector.uuids.get(0).unwrap().clone();
            //mandant_object = mandant_collector.result.get(&0).unwrap();
            gen_birds_for_mandant(&mut birds_collector, &mandant_uuid);
        }
    }
    nestboxes_collector.flush();
    users_collector.flush();
    generate_nestboxes_additionals(
        &users_collector,
        &nestboxes_collector,
        &birds_collector,
        &mut geolocations_collector,
        &mut breeds_collector,
    );
    geolocations_collector.flush();
    breeds_collector.flush();
    Ok(())
}

fn gen_birds_for_mandant(birds_collector: &mut Collector, mandant_uuid: &str) {
    for b in 0..150 {
        birds_collector.append_doc(
            doc! { "uuid": Uuid::new_v4().to_string(), "bird": format!("bird_{}", b), "mandant_uuid": mandant_uuid},
        );
    }
    birds_collector.flush();
}

fn get_password_and_salt() -> (Uuid, String) {
    let user_password_salt = Uuid::new_v4();
    let mut hasher = Sha3_256::new();
    let password_with_salt = format!("{}_{}", "secretbird", user_password_salt.to_string());
    hasher.update(password_with_salt);
    let password_hash = hex::encode(hasher.finalize());
    (user_password_salt, password_hash)
}

fn generate_nestboxes_additionals(
    users_collector: &Collector,
    nestboxes_collector: &Collector,
    birds_collector: &Collector,
    geolocations_collector: &mut Collector,
    breeds_collector: &mut Collector,
) {
    let number_of_birds = birds_collector.uuids.len();
    let mut rng = rand::thread_rng();
    for (c, nestbox_uuid) in nestboxes_collector.uuids.iter().enumerate() {
        let user_uuid = users_collector.uuids.get(c).unwrap();
        for _b in 0..6 {
            let longitude = random_longitude(-180.0, 180.0);
            let latitude = random_latitude(-90.0, 90.0);
            geolocations_collector.append_doc(doc! { "uuid": Uuid::new_v4().to_string(),
            "nestbox_uuid": &nestbox_uuid,
            "from_date": Utc::now(),
            "until_date": Utc::now(),
            "position": {"type": "point", "coordinates": [ longitude, latitude ]}});
            breeds_collector.append_doc(doc! {
            "uuid": Uuid::new_v4().to_string(),
            "nestbox_uuid": &nestbox_uuid,
            "user_uuid": &user_uuid,
            "discovery_date": Utc::now(),
            "bird_uuid": birds_collector.uuids.get(rng.gen_range(0..number_of_birds)).unwrap()});
        }
    }
}

fn random_latitude(from: f32, until: f32) -> f32 {
    // Valid latitude values are between -90 and 90, both inclusive.
    get_random_range(from, until, -90.0, 90.0)
}

fn random_longitude(from: f32, until: f32) -> f32 {
    // Valid longitude values are between -180 and 180, both inclusive.
    get_random_range(from, until, -180.0, 180.0)
}

fn get_random_range(from: f32, until: f32, valid_min: f32, valid_max: f32) -> f32 {
    let mut from_cleaned = from;
    let mut until_cleaned = until;
    if from < valid_min {
        from_cleaned = valid_min;
    }
    if until > valid_max {
        until_cleaned = valid_max;
    }
    let mut rng = rand::thread_rng();
    rng.gen_range(from_cleaned..until_cleaned)
}

#[cfg(test)]
mod tests {

    use super::*;
    const INSERTED_RECORDS: usize = 12;
    const DB_URI: &str = "mongodb://127.0.0.1:27017/?w=majority";
    const DATABASE: &str = "nestbox_bouncycastle";
    // Note: any case below tests only if the number of records in the db
    // are greater after the test then before. So the test only asserts
    // that an insertion has been done.
    #[test]
    fn test_insertion_collection_mandants() {
        db_assert(COL_MANDANTS);
    }
    #[test]
    fn test_insertion_collection_nestboxes() {
        db_assert(COL_NESTBOXES);
    }
    #[test]
    fn test_insertion_collection_breeds() {
        db_assert(COL_BREEDS);
    }

    #[test]
    fn test_insertion_collection_birds() {
        db_assert(COL_BIRDS);
    }

    #[test]
    fn test_insertion_collection_users() {
        db_assert(COL_USERS);
    }

    #[test]
    fn test_insertion_collection_geolocations() {
        db_assert(COL_GEOLOCATIONS);
    }

    fn db_assert(collection: &str) {
        let client = match Client::with_uri_str(DB_URI) {
            Ok(c) => c,
            _ => return,
        };
        let database = client.database(DATABASE);
        let mandants_collection = database.collection(collection);
        let mandants_res_before_test = mandants_collection.count_documents(doc! {}, None);
        let _result = populate_db(DB_URI, DATABASE, INSERTED_RECORDS as i32);
        let mandants_res = mandants_collection.count_documents(doc! {}, None);
        assert_eq!(
            mandants_res.unwrap() > mandants_res_before_test.unwrap(),
            true
        );
    }
}
