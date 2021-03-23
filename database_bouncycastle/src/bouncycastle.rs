use mongodb::{bson::doc};
use mongodb::sync::Client;
use chrono::prelude::*;

type InsertManyType = Result<mongodb::results::InsertManyResult, mongodb::error::Error>;
type VecDocType = Vec<mongodb::bson::Document>;

pub fn poplate_db() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://127.0.2.15:27017/?w=majority")?;

    let database = client.database("nestbox");
    let breeds = database.collection("breeds");
    
    let mut breed_docs: VecDocType = Vec::new();
    for i in  0..100000 {
        breed_docs.push(doc!{"name": format!("breed_eleven {}", i), "date": Utc::now()});
        if i % 10000 == 0 {
            let _result = write_to_db(&breeds, &mut breed_docs)?;
            for (id, object_id) in _result.inserted_ids {
                println!("id {:#?}, object_id {:#?}", id, object_id);
            }
            
        } 
    } 
    if breed_docs.len() > 0 {
        let _result = write_to_db(&breeds, &mut breed_docs)?;
    }
    Ok(())
}

fn write_to_db(breeds: &mongodb::sync::Collection, breed_docs: &mut VecDocType) -> InsertManyType {
    let result = breeds.insert_many(breed_docs.clone().into_iter(), None)?;
    breed_docs.clear();
    Ok(result)
}