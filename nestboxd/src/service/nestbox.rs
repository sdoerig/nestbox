use bson::{doc, Document};
use mongodb::{error::Error, Collection, Database};
use crate::controller::{req_structs::NestboxReq, utilities::SessionObject};

#[derive(Clone)]
pub struct NestboxService {
    collection: Collection,
}
const NESTBOX: &str = "nestboxes";

impl NestboxService {
    pub fn new(db: &Database) -> NestboxService {
        NestboxService { collection: db.collection(NESTBOX) }
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

    pub async fn append_image_by_uuid(&self, uuid: &str, images: &[String]) -> bool {
        //let update = doc!
        let mut update_res = true;
        for image in images {
            let result = self
                .collection
                .update_one(
                    doc! {"uuid": uuid},
                    doc! {"$addToSet": doc!{"images":image}},
                    None,
                )
                .await;
            match result {
                Ok(_r) => update_res = true,
                _error => {
                    update_res = false;
                    break;
                }
            }
        }
        update_res
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
        let db = fetch_db().await;
        let nestbox_service = NestboxService::new(&db);

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
        let db = fetch_db().await;
        let nestbox_service = NestboxService::new(&db);
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
        let db = fetch_db().await;
        let nestbox_service = NestboxService::new(&db);
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

    async fn fetch_db() -> Database {
        let client_options_future = ClientOptions::parse("mongodb://localhost:27017");
        let client_options = client_options_future.await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("nestbox_testing");
        db
    }
}
