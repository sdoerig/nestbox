use bson::{doc, Document};

use crate::controller::breed::{BreedReq};
use crate::controller::utilities::{DocumentResponse, PagingQuery};
use futures::{executor::block_on, StreamExt};
use mongodb::{Collection, error::Error, options::{AggregateOptions, FindOptions}};
use super::service_helper as sa;

#[derive(Clone)]
pub struct BreedService {
    collection: Collection,
}


impl BreedService {
    pub fn new(collection: Collection) -> BreedService {
        BreedService { collection }
    }

    pub async fn get_by_nestbox_uuid(
        &self,
        req: &BreedReq,
        paging: &PagingQuery,
    ) -> DocumentResponse {
        
        let res = self
            .collection
            .aggregate(vec! [
                doc! {"$match": {"nestbox_uuid": {"$eq": &req.uuid}}}, 
                doc! {"$skip": (paging.page_limit * (paging.page_number -1))}, 
                doc!{"$limit": paging.page_limit}, 
                doc!{"$lookup": {"from": "birds", "localField": "bird_uuid", "foreignField": "uuid", "as": "bird"}}], None);
        let counted_documents_res = self.get_by_nestbox_count(&req.uuid).await;

        let blocked_res = block_on(res);

       let documents = sa::read_mongodb_cursor(blocked_res).await;
        let counted_documents = match counted_documents_res {
            Ok(i) => i,
            Err(_e) => 0,
        };

        DocumentResponse::new(documents, counted_documents, paging)
    }

    pub async fn get_by_nestbox_count(&self, nestbox_uuid: &str) -> Result<i64, Error> {
        self.collection
            .count_documents(doc! {"nestbox_uuid": nestbox_uuid}, None)
            .await
    }
}

