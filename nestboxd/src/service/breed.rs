use bson::doc;
use chrono::Utc;
use uuid::Uuid;

use super::service_helper as sa;
use crate::controller::{
    req_structs::BirdReq,
    utilities::{DocumentResponse, PagingQuery},
};
use crate::controller::{req_structs::NestboxReq, utilities::SessionObject};
use futures::executor::block_on;
use mongodb::{error::Error, Collection};

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
        session_obj: &SessionObject,
        req: &NestboxReq,
        paging: &PagingQuery,
    ) -> DocumentResponse {
        let mut projection =
            doc! {"$project": {"_id": 0, "mandant_uuid": 0, "user_uuid": 0, "bird_uuid": 0}};
        if session_obj.is_valid_session() {
            projection = doc! {"$project": {"_id": 0, "mandant_uuid": 0, "bird_uuid": 0}};
        }
        let res = self.collection.aggregate(
            vec![
                doc! {"$match": {"nestbox_uuid": {"$eq": &req.uuid}}},
                doc! {"$skip": (paging.page_limit * (paging.page_number -1))},
                doc! {"$limit": paging.page_limit},
                doc! {"$lookup": {
                "from": "birds",
                "let": {
                  "breeds_bird_uuid": "$bird_uuid" },
                "pipeline":[
                  {
                    "$match": {
                      "$expr": {
                        "$eq": [
                          "$$breeds_bird_uuid", "$uuid"
                        ]
                      }
                    }
                  },
                  {
                    "$project": {
                       "_id":0, "uuid": 1, "bird": 1
                    }
                  }
                ], "as": "bird"}},
                projection,
            ],
            None,
        );
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

    pub async fn post_breed(
        &self,
        session_obj: &SessionObject,
        nestbox_req: &NestboxReq,
        bird: &BirdReq,
    ) {
        let breed = doc! {
          "uuid": Uuid::new_v4().to_string(),
          "nestbox_uuid": &nestbox_req.uuid,
          "user_uuid": session_obj.get_user_uuid(),
          "discovery_date": Utc::now(),
          "bird_uuid": &bird.uuid};
        let res = self.collection.insert_one(breed, None).await;
    }
}
