use futures::stream::StreamExt;
use mongodb::{
     bson::{doc, Document},
     error::Result,
     Client, Collection
};

#[derive(Clone)]
pub struct BreedService {
    collection: Collection,
}

impl BreedService {
    pub fn new(collection: Collection) -> BreedService {
        BreedService { collection }
    }

    pub async fn get_by_nestbox(&self, nestbox: &Document) {
        let res = self.collection.find(Some(doc! {"nestbox.$id": nestbox.get("_id").unwrap()}), None).await;
        let cursor = match res {
            Ok(c) => c,
            Err(e) => return
        };
        let results: Vec<Result<Document>> = cursor.collect().await;
        
        
       
    }

    
}
