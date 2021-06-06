use bson::doc;
use chrono::Utc;
use uuid::Uuid;

use super::service_helper as sa;
use crate::controller::{
    req_structs::BirdReq,
    utilities::{DocumentResponse, PagingQuery},
};
use crate::controller::{req_structs::NestboxReq, utilities::SessionObject};
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
        let res = self
            .collection
            .aggregate(
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
            )
            .await;
        let counted_documents_res = self.get_by_nestbox_count(&req.uuid).await;

        let documents = sa::read_mongodb_cursor(res).await;
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

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::{options::ClientOptions, Client, Collection};

    // Nestbox from mandant_uuid 4ac9971c-91de-455c-a1fd-4b9dfb862cee
    const NESTBOX_UUID_OK: &str = "a446545d-f594-4eb5-96b4-c2312554050c";

    #[actix_rt::test]
    async fn test_service_breed_get_by_nestbox_uuid() {
        let nestboxes_col = fetch_collection("breeds").await;
        let breeds_service = BreedService::new(nestboxes_col);
        // Creating a mock session object
        let session_doc = doc! { "mandant_uuid" : "4ac9971c-91de-455c-a1fd-4b9dfb862cee",
        "username" : "fg_11", "uuid" : "15eaa6ca-4797-442b-b6c9-f1e7a1f3416d",
        "lastname" : "Gucker",
        "firstname" : "Fritz", "email" : "email_11@birdwatch.ch",
        "session_key" : "0e16a457-d957-431a-ba9e-ff3a961ed60e" };

        let session = SessionObject::new(Ok(Some(session_doc)));
        let nestbox_req = NestboxReq {
            uuid: String::from(NESTBOX_UUID_OK),
        };
        let breeds_response = breeds_service
            .get_by_nestbox_uuid(
                &session,
                &nestbox_req,
                &PagingQuery {
                    page_limit: 2,
                    page_number: 1,
                },
            )
            .await;
        assert_eq!(breeds_response.counted_documents, 6_i64);
        assert_eq!(breeds_response.page_number, 1_i64);
        assert_eq!(breeds_response.pages, 3_i64);
        assert_eq!(breeds_response.page_limit, 2_i64);
    }

    async fn fetch_collection(users_col: &str) -> Collection {
        let client_options_future = ClientOptions::parse("mongodb://localhost:27017");
        let client_options = client_options_future.await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("nestbox_testing");
        db.collection(users_col)
    }
}
