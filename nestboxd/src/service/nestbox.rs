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
            )
            .await;
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::{options::ClientOptions, Client, Collection};

    // Nestbox from mandant_uuid 4ac9971c-91de-455c-a1fd-4b9dfb862cee
    const NESTBOX_UUID_OK: &str = "a446545d-f594-4eb5-96b4-c2312554050c";
    const NESTBOX_UUID_NOK: &str = "74a0d653-f93a-4383-822f-8f55ab853fca";

    #[actix_rt::test]
    async fn test_service_nestbox_get_by_uuid_ok() {
        let nestboxes_col = fetch_collection("nestboxes").await;
        let nestbox_service = NestboxService::new(nestboxes_col);

        let nestbox = nestbox_service.get_by_uuid(NESTBOX_UUID_OK).await.unwrap();
        assert_eq!(
            nestbox
                .unwrap()
                .get("uuid")
                .unwrap()
                .to_string()
                .replace('"', ""),
            String::from(NESTBOX_UUID_OK)
        );
    }
    #[actix_rt::test]
    async fn test_service_nestbox_get_by_mandant_uuid_ok() {
        let nestboxes_col = fetch_collection("nestboxes").await;
        let nestbox_service = NestboxService::new(nestboxes_col);
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
        let nestbox = nestbox_service
            .get_by_mandant_uuid(&session, &nestbox_req)
            .await
            .unwrap();
        assert_eq!(
            nestbox
                .unwrap()
                .get("uuid")
                .unwrap()
                .to_string()
                .replace('"', ""),
            String::from(NESTBOX_UUID_OK)
        );
    }

    #[actix_rt::test]
    async fn test_service_nestbox_get_by_mandant_uuid_nok() {
        let nestboxes_col = fetch_collection("nestboxes").await;
        let nestbox_service = NestboxService::new(nestboxes_col);
        // Creating a mock session object
        let session_doc = doc! { "mandant_uuid" : "4ac9971c-91de-455c-a1fd-4b9dfb862cee", "username" : "fg_11", "uuid" : "15eaa6ca-4797-442b-b6c9-f1e7a1f3416d", "lastname" : "Gucker", "firstname" : "Fritz", "email" : "email_11@birdwatch.ch", "session_key" : "0e16a457-d957-431a-ba9e-ff3a961ed60e" };

        let session = SessionObject::new(Ok(Some(session_doc)));
        let nestbox_req = NestboxReq {
            uuid: String::from(NESTBOX_UUID_NOK),
        };
        let nestbox = nestbox_service
            .get_by_mandant_uuid(&session, &nestbox_req)
            .await
            .unwrap();
        assert_eq!(nestbox, None);
    }

    async fn fetch_collection(users_col: &str) -> Collection {
        let client_options_future = ClientOptions::parse("mongodb://localhost:27017");
        let client_options = client_options_future.await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("nestbox_testing");
        db.collection(users_col)
    }
}
