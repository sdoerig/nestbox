use bson::{doc, Document};
use futures::executor::block_on;
use mongodb::{error::Error, Collection};


#[derive(Clone)]
pub struct NestboxService {
    collection: Collection,
}

impl NestboxService {
    pub fn new(collection: Collection) -> NestboxService {
        NestboxService { collection }
    }

    pub fn get(&self) -> Result<Option<Document>, Error> {
        let res = self.collection.find_one(doc! {}, None);
        block_on(res)
    }

    pub fn get_by_uuid(&self, uuid: &str) -> Result<Option<Document>, Error> {
        let res = self.collection.find_one(doc! {"uuid": uuid}, None);
        block_on(res)
    }
}