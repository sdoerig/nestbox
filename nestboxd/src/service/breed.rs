use bson::{doc, Document};

use futures::{FutureExt, executor::block_on};
use mongodb::{Collection, Cursor, error::Error};

#[derive(Clone)]
pub struct BreedService {
    collection: Collection,
}

impl BreedService {
    pub fn new(collection: Collection) -> BreedService {
        BreedService { collection }
    }

    pub fn get_by_nestbox(&self, nestbox: &Document) -> Result<Cursor<Document>, Error> {
        let mut results_doc: Vec<Document> = Vec::new();
        let res = self.collection.find(Some(doc! {"nestbox.$id": nestbox.get("_id").unwrap()}), None);
        block_on(res)
        
       
    }

    
}
