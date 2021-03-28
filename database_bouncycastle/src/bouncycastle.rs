use std::collections::{HashMap, hash_map::RandomState};

use mongodb::{bson::{Bson, doc}, sync::Collection};
use mongodb::sync::Client;
use chrono::prelude::*;
use uuid::Uuid;

type InsertManyType = Result<mongodb::results::InsertManyResult, mongodb::error::Error>;
type VecDocType = Vec<mongodb::bson::Document>;
const STEP_SIZE: usize = 100000;

struct Collector {
    docs: VecDocType,
    collection: Collection,
    pub result: HashMap<usize, Bson, RandomState>
}

impl Collector {
    pub fn new(collection: Collection) -> Self {
        Collector{docs: Vec::new(), collection: collection, result: HashMap::new()}
    }

    pub fn append_doc(&mut self, doc: mongodb::bson::Document){
        self.docs.push(doc);
        if self.docs.len() > STEP_SIZE {
            self.write_to_db();
        }
    }

    fn write_to_db(&mut self) {
        self.result = match self.collection.insert_many(self.docs.clone().into_iter(), None) {
            Ok(s) => s.inserted_ids,
            _ => HashMap::new()
        }; 
        self.docs.clear();
    }

    pub fn flush(&mut self) {
        if self.docs.len() > 0 {
            return self.write_to_db();
        }
    }
    
}

pub fn poplate_db(db_uri: &str, records_to_insert: i32) -> mongodb::error::Result<()> {
    let client = Client::with_uri_str(db_uri)?;

    let database = client.database("nestbox");
    let breeds = database.collection("breeds");
    let nestboxes = database.collection("nestboxes");
    let mandant = database.collection("mandant");
    let mut nestboxes_collector = Collector::new(database.collection("nestboxes"));
    let mut mandant_collector = Collector::new(database.collection("mandant")); 
    mandant_collector.append_doc(doc!{"name": "BirdLife",  "website": "https://www.birdwatcher.ch", "email": "bird@iseeyou.ch"});
       
    let mut docs: VecDocType = Vec::new();
    docs.push(doc!{"name": "BirdLife",  "website": "https://www.birdwatcher.ch", "email": "bird@iseeyou.ch"});
    let mut mandant_result = write_to_db(&mandant, &mut docs)?;
    println!("mandant {:#?}", mandant_result);
    mandant_collector.flush();
    let mut mandant_object = mandant_result.inserted_ids.get(&0).unwrap();
    for i in 0..records_to_insert as usize {
        let uuid_4 = Uuid::new_v4().to_string(); 

        nestboxes_collector.append_doc(doc!{"public": true, "uuid": uuid_4, "mandant": mandant_object});
        if i % 100 == 0 {
            let mut mandant_docs: VecDocType = vec![doc!{"name": format!("BirdLife {}", i),  "website": "https://www.birdwatcher.ch", "email": "bird@iseeyou.ch"}];
            mandant_result = write_to_db(&mandant, &mut mandant_docs)?;
            mandant_object = mandant_result.inserted_ids.get(&0).unwrap();
        }
        if i % STEP_SIZE == 0 {
            let _result = write_to_db(&nestboxes, &mut docs);
        }
        
    }
    if docs.len() > 0 {
        let _result = write_to_db(&nestboxes, &mut docs);
    }


    for i in  0..records_to_insert as usize {
        docs.push(doc!{"name": format!("breed_eleven {}", i), "date": Utc::now()});
        if i % STEP_SIZE == 0 {
            let _result = write_to_db(&breeds, &mut docs)?;
            //for (id, object_id) in _result.inserted_ids {
            //    println!("id {:#?}, object_id {:#?}", id, object_id);
            //}
            
        } 
    } 
    if docs.len() > 0 {
        let _result = write_to_db(&breeds, &mut docs)?;
    }
    Ok(())
}

fn write_to_db(collection: &mongodb::sync::Collection, docs: &mut VecDocType) -> InsertManyType {
    let result = collection.insert_many(docs.clone().into_iter(), None)?;
    docs.clear();
    Ok(result)
}


pub fn poplate_db_new(db_uri: &str, records_to_insert: i32) -> mongodb::error::Result<()> {
    let client = Client::with_uri_str(db_uri)?;

    let database = client.database("nestbox");
    let mut nestboxes_collector = Collector::new(database.collection("nestboxes"));
    let mut mandant_collector = Collector::new(database.collection("mandant")); 

    mandant_collector.append_doc(doc!{"name": "BirdLife",  "website": "https://www.birdwatcher.ch", "email": "bird@iseeyou.ch"});
    for i in 0..records_to_insert as usize {
        mandant_collector.flush();
        
    }
    
    Ok(())
}

