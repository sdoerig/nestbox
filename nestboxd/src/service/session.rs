use crate::controller::utilities::SessionObject;
use actix_web::HttpRequest;
use bson::{doc, Document};
use chrono::Utc;
use mongodb::{Collection};
use uuid::Uuid;

const HTTP_AUTHORIZATION: &str = "Authorization";

#[derive(Clone)]
pub struct SessionService {
    collection: Collection,
}

impl SessionService {
    pub fn new(collection: Collection) -> SessionService {
        SessionService { collection }
    }

    pub async fn create_session(&self, user_obj: Document) -> String {
        let session_id = Uuid::new_v4().to_string();
        //let mut doc = Document::new();
        //doc.
        let mut session_obj = user_obj;
        session_obj.remove("_id");
        session_obj.insert("session_key", &session_id);
        session_obj.insert("session_created_at", Utc::now());
        self.remove_session(&session_obj).await;
        let _session = self.collection.insert_one(session_obj, None).await;
        session_id
    }

    async fn remove_session(&self, session_obj: &Document) {
        let bson = session_obj.get("username");
        let username = match bson {
            Some(b) => b.as_str().unwrap(),
            None => "n.a.",
        };
        let _removed_session = self
            .collection
            .delete_many(doc! {"username": username}, None)
            .await;
    }

    pub async fn remove_session_by_username(&self, username: &str) {
        let _removed_session = self
            .collection
            .delete_many(doc! {"username": username}, None)
            .await;
    }

    pub async fn validate_session(&self, http_req: &HttpRequest) -> SessionObject {
        let session_token = match http_req.headers().get(HTTP_AUTHORIZATION) {
            Some(t) => t.to_str(),
            None => Ok("n.a."),
        };
        let session_obj = self
            .collection
            .find_one(
                doc! {"session_key": session_token.unwrap().replace("Basic ", "")},
                None,
            )
            .await;

        SessionObject::new(session_obj)
    }
}
