//! # Database Bouncy Castle
//! As the name indicated, this programm is just for playing. Its main purpose is
//! to give my a feeling of the data model and stock the database the rest
//! will hopefully develop on with a reasonable amount of records.
//! This because I do feel unconfortable if developing something on
//! an empty database, because real life does not take place on an empty database.
//!
//! There are also no tests - I might write some later on - but as long as it fills
//! the database with records I don't see a need to write some.
//!
use chrono::prelude::*;
use mongodb::sync::Client;
use mongodb::{
    bson::{doc, Bson},
    sync::Collection,
};
use rand::Rng;
use sha3::{Digest, Sha3_256};
use std::collections::{hash_map::RandomState, HashMap};
use uuid::Uuid;

type VecDocType = Vec<mongodb::bson::Document>;
const STEP_SIZE: usize = 10000;

enum CollectorState {
    Flushed,
    Accumulating,
}
struct Collector {
    docs: VecDocType,
    collection: Collection,
    pub result: HashMap<usize, Bson, RandomState>,
}

impl Collector {
    // Collects the generated records and if the STEPSIZE is reached
    // writes it to the mongodb collection.
    pub fn new(collection_store: Collection) -> Self {
        Collector {
            docs: Vec::new(),
            collection: collection_store,
            result: HashMap::new(),
        }
    }

    pub fn append_doc(&mut self, doc: mongodb::bson::Document) -> CollectorState {
        self.docs.push(doc);
        if self.docs.len() > STEP_SIZE {
            self.write_to_db();
            return CollectorState::Flushed;
        }
        CollectorState::Accumulating
    }

    fn write_to_db(&mut self) {
        self.result = match self.collection.insert_many(self.docs.drain(..), None) {
            Ok(s) => s.inserted_ids,
            _ => HashMap::new(),
        };
    }

    pub fn flush(&mut self) {
        if !self.docs.is_empty() {
            self.write_to_db();
        }
    }
}

pub fn poplate_db(db_uri: &str, records_to_insert: i32) -> mongodb::error::Result<()> {
    let client = Client::with_uri_str(db_uri)?;

    let database = client.database("nestbox");
    let mut nestboxes_collector = Collector::new(database.collection("nestboxes"));
    let mut mandant_collector = Collector::new(database.collection("mandants"));
    let mut breeds_collector = Collector::new(database.collection("breeds"));
    let mut users_collector = Collector::new(database.collection("users"));
    let mut geolocations_collector = Collector::new(database.collection("geolocations"));
    mandant_collector.append_doc(doc!{"name": "BirdLife",  "website": "https://www.birdwatcher.ch", "email": "bird@iseeyou.ch"});
    mandant_collector.flush();

    let mut mandant_object = mandant_collector.result.get(&0).unwrap();
    for i in 0..records_to_insert as usize {
        let (user_password_salt, password_hash) = fun_name();
        users_collector.append_doc(doc!{
            "mandant_id": mandant_object,
            "lastname": "Gucker",
            "firstname":"Fritz",
            "email": format!("email_{}@birdwatch.ch", i),
            "password_hash": password_hash,
            "salt": user_password_salt.to_string()});
        let nestbox_flushed = match nestboxes_collector.append_doc(
            doc! {"public": true, "uuid": Uuid::new_v4().to_string(), "mandant": mandant_object, "created_at": Utc::now()},
        ) {
            CollectorState::Flushed => true,
            CollectorState::Accumulating => false,
        };
        if nestbox_flushed {
            generate_nestboxes_additionals(
                &users_collector,
                &nestboxes_collector,
                &mut geolocations_collector,
                &mut breeds_collector,
            );
        }
        if i % 100 == 0 {
            mandant_collector.append_doc(
                doc!{"name": format!("BirdLife {}", i),  "website": "https://www.birdwatcher.ch", "email": "bird@iseeyou.ch"});
            mandant_collector.flush();
            mandant_object = mandant_collector.result.get(&0).unwrap();
        }
    }
    nestboxes_collector.flush();
    users_collector.flush();
    generate_nestboxes_additionals(
        &users_collector,
        &nestboxes_collector,
        &mut geolocations_collector,
        &mut breeds_collector,
    );
    geolocations_collector.flush();
    breeds_collector.flush();
    Ok(())
}

fn fun_name() -> (Uuid, String) {
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
    geolocations_collector: &mut Collector,
    breeds_collector: &mut Collector,
) {
    for (_c, nestbox_object) in nestboxes_collector.result.iter() {
        let user_object = users_collector.result.get(&_c).unwrap();
        for _b in 0..6 {
            let longitude = random_longitude(-180.0, 180.0);
            let latitude = random_latitude(-90.0, 90.0);
            geolocations_collector.append_doc(doc! {"nestbox_id": nestbox_object,
            "from_date": 0,
            "until_date": 0,
            "position": {"type": "point", "coordinates": [ longitude, latitude ]}});
            breeds_collector
                .append_doc(doc! {
                    "nestbox_id": nestbox_object, 
                    "user_id": user_object, 
                    "discovery_date": Utc::now()});
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
