use bson::{doc, Document};
use futures::executor::block_on;
use mongodb::{error::Error, Collection};

use crate::controller::{req_structs::NestboxReq, utilities::SessionObject};

#[derive(Clone)]
pub struct NestboxService {
    collection: Collection,
}

impl NestboxService {
    pub fn new(collection: Collection) -> NestboxService {
        NestboxService { collection }
    }

    pub async fn get_by_uuid(&self, uuid: &str) -> Result<Option<Document>, Error> {
        let res = self.collection.find_one(doc! {"uuid": uuid}, None).await;
        res
    }

    pub async fn get_by_mandant_uuid(
        &self,
        session: &SessionObject,
        nestbox_req: &NestboxReq,
    ) -> Result<Option<Document>, Error> {
        let res = self
            .collection
            .find_one(
                doc! {"uuid": &nestbox_req.uuid,
                "mandant_uuid": session.get_mandant_uuid()},
                None,
            );
        block_on(res)
    }
}
