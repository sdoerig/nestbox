use bson::{doc, Document};
use mongodb::Collection;
use uuid::Uuid;

use crate::controller::user;

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
        self.remove_session(&session_obj).await;
        let session = self.collection.insert_one(session_obj, None).await;
        session_id
    }

    async fn remove_session(&self, session_obj: &Document) {
        let bson = session_obj.get("email");
        let email = match bson {
            Some(b) => b.as_str().unwrap(),
            None => "n.a.",
        };
        let _removed_session = self
            .collection
            .delete_many(doc! {"email": email}, None)
            .await;
    }
}
