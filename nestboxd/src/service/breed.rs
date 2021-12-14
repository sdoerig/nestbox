use mongodb::bson::{doc, DateTime, Document};
//use chrono::Utc;
use uuid::Uuid;

use super::res_structs::{BreedResponse, MapDocument};
use super::service_helper as sa;
use crate::controller::{req_structs::BirdReq, utilities::PagingQuery};
use crate::controller::{req_structs::NestboxReq, utilities::SessionObject};
use mongodb::{error::Error, Collection, Database};

const BREEDS: &str = "breeds";

#[derive(Clone)]
pub struct BreedService {
    collection: Collection<Document>,
}

impl BreedService {
    pub fn new(db: &Database) -> Self {
        BreedService {
            collection: db.collection(BREEDS),
        }
    }

    pub async fn get_by_nestbox_uuid(
        &self,
        session_obj: &SessionObject,
        req: &NestboxReq,
        paging: &PagingQuery,
    ) -> (Vec<BreedResponse>, i64) {
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

        let mut breed_responses: Vec<BreedResponse> = Vec::new();
        for d in documents {
            breed_responses.push(BreedResponse::map_doc(&d));
        }

        let counted_documents = match counted_documents_res {
            Ok(i) => i,
            Err(_e) => 0,
        };

        (breed_responses, counted_documents as i64)
    }

    pub async fn get_by_nestbox_count(&self, nestbox_uuid: &str) -> Result<u64, Error> {
        self.collection
            .count_documents(doc! {"nestbox_uuid": nestbox_uuid}, None)
            .await
    }

    pub async fn post_breed(
        &self,
        session_obj: &SessionObject,
        nestbox_req: &NestboxReq,
        bird: &BirdReq,
    ) -> std::result::Result<BreedResponse, Error> {
        let breed = doc! {
        "uuid": Uuid::new_v4().to_string(),
        "nestbox_uuid": &nestbox_req.uuid,
        "user_uuid": session_obj.get_user_uuid(),
        "discovery_date": DateTime::now(),
        "bird_uuid": &bird.bird_uuid};
        match self.collection.insert_one(&breed, None).await {
            Ok(_o) => Ok(BreedResponse::map_doc(&breed)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::{options::ClientOptions, Client};

    // Nestbox from mandant_uuid 4ac9971c-91de-455c-a1fd-4b9dfb862cee
    const NESTBOX_UUID_OK: &str = "a446545d-f594-4eb5-96b4-c2312554050c";

    #[actix_rt::test]
    async fn test_service_breed_get_by_nestbox_uuid() {
        let db = fetch_db().await;
        let breeds_service = BreedService::new(&db);
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
        let (_documents, counted_documents) = breeds_service
            .get_by_nestbox_uuid(
                &session,
                &nestbox_req,
                &PagingQuery {
                    page_limit: 2,
                    page_number: 1,
                },
            )
            .await;
        assert_eq!(counted_documents, 6_i64);
    }

    async fn fetch_db() -> Database {
        let client_options_future = ClientOptions::parse("mongodb://localhost:27017");
        let client_options = client_options_future.await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        client.database("nestbox_testing")
    }
}
