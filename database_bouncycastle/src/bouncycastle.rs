use mongodb::{bson::doc};
use mongodb::sync::Client;
use chrono::prelude::*;
use uuid::Uuid;

type InsertManyType = Result<mongodb::results::InsertManyResult, mongodb::error::Error>;
type VecDocType = Vec<mongodb::bson::Document>;
const STEP_SIZE: i32 = 100000;

pub fn poplate_db(db_uri: &str, records_to_insert: i32) -> mongodb::error::Result<()> {
    let client = Client::with_uri_str(db_uri)?;

    let database = client.database("nestbox");
    let breeds = database.collection("breeds");
    let nestboxes = database.collection("nestboxes");
    let mandant = database.collection("mandant");
        
    let mut docs: VecDocType = Vec::new();
    docs.push(doc!{"name": "BirdLife",  "website": "https://www.birdwatcher.ch", "email": "bird@iseeyou.ch"});
    let mut mandant_result = write_to_db(&mandant, &mut docs)?;
    println!("mandant {:#?}", mandant_result);
    let mut mandant_object = mandant_result.inserted_ids.get(&0).unwrap();
    for i in 0..records_to_insert {
        let uuid_4 = Uuid::new_v4().to_string(); 

        docs.push(doc!{"public": true, "uuid": uuid_4, "mandant": mandant_object});
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


    for i in  0..records_to_insert {
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