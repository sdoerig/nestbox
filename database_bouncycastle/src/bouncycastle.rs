use rand::Rng;
use std::collections::{hash_map::RandomState, HashMap};

use chrono::prelude::*;
use mongodb::sync::Client;
use mongodb::{
    bson::{doc, Bson},
    sync::Collection,
};
use uuid::Uuid;

type InsertManyType = Result<mongodb::results::InsertManyResult, mongodb::error::Error>;
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
    pub fn new(collection: Collection) -> Self {
        Collector {
            docs: Vec::new(),
            collection: collection,
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
        self.result = match self
            .collection
            .insert_many(self.docs.clone().into_iter(), None)
        {
            Ok(s) => s.inserted_ids,
            _ => HashMap::new(),
        };
        self.docs.clear();
    }

    pub fn flush(&mut self) {
        if self.docs.len() > 0 {
            self.write_to_db();
        }
    }
}

pub fn poplate_db(db_uri: &str, records_to_insert: i32) -> mongodb::error::Result<()> {
    let client = Client::with_uri_str(db_uri)?;

    let database = client.database("nestbox");
    let mut nestboxes_collector = Collector::new(database.collection("nestboxes"));
    let mut mandant_collector = Collector::new(database.collection("mandant"));
    let mut breeds_collector = Collector::new(database.collection("breeds"));
    //let mut users_collector = Collector::new(database.collection("users"));
    let mut geolocations_collector = Collector::new(database.collection("geolocations"));
    mandant_collector.append_doc(doc!{"name": "BirdLife",  "website": "https://www.birdwatcher.ch", "email": "bird@iseeyou.ch"});
    mandant_collector.flush();
    let mut mandant_object = mandant_collector.result.get(&0).unwrap();

    for i in 0..records_to_insert as usize {
        let nestbox_flushed = match nestboxes_collector.append_doc(
            doc! {"public": true, "uuid": Uuid::new_v4().to_string(), "mandant": mandant_object, "created_at": Utc::now()},
        ) {
            CollectorState::Flushed => true,
            CollectorState::Accumulating => false,
        };
        if nestbox_flushed {
            generate_nestboxes_additionals(&nestboxes_collector, &mut geolocations_collector, &mut breeds_collector);
        }
        if i % 100 == 0 {
            mandant_collector.append_doc(
                doc!{"name": format!("BirdLife {}", i),  "website": "https://www.birdwatcher.ch", "email": "bird@iseeyou.ch"});
            mandant_collector.flush();
            mandant_object = mandant_collector.result.get(&0).unwrap();
        }
    }
    nestboxes_collector.flush();
    generate_nestboxes_additionals(&nestboxes_collector, &mut geolocations_collector, &mut breeds_collector);
    geolocations_collector.flush();
    breeds_collector.flush();
    Ok(())
}

fn generate_nestboxes_additionals(nestboxes_collector: &Collector, geolocations_collector: &mut Collector, breeds_collector: &mut Collector) {
    for (_c, nestbox_object) in nestboxes_collector.result.iter() {
        for _b in 0..6 {
            geolocations_collector.append_doc(
                doc!{"nestbox_id": nestbox_object, 
                "from_date": 0, 
                "until_date": 0, 
                "longitude": random_longitude(-180.0, 180.0), 
                "latitude": random_latitude(-90.0, 90.0)});
            breeds_collector.append_doc(doc! {"nestbox_id": nestbox_object});
        }
    }
}

fn random_latitude(from: f32, until: f32) -> f32 {
    //Valid latitude values are between -90 and 90, both inclusive.
    get_random_range(from, until, -90.0, 90.0)
}

fn random_longitude(from: f32, until: f32) -> f32 {
    //Valid longitude values are between -180 and 180, both inclusive.
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