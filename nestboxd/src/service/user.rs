use mongodb::bson::{doc, Document};
use mongodb::{Collection, Database};

use sha3::{Digest, Sha3_256};

const USERS: &str = "users";

#[derive(Clone)]
pub struct UserService {
    collection: Collection<Document>,
}

impl UserService {
    pub fn new(db: &Database) -> UserService {
        UserService {
            collection: db.collection(USERS),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Option<Document> {
        let user_res = self
            .collection
            .find_one(doc! {"username": username}, None)
            .await
            .ok()?;
        //block_on(user_res);
        let userobj = match user_res {
            Some(u) => u,
            None => return None,
        };

        let mut pw_hash_salt: Vec<String> = Vec::new();

        for key in &["password_hash", "salt"] {
            let bson = userobj.get(key);
            let string = match bson {
                Some(b) => b.as_str()?,
                None => return None,
            };
            pw_hash_salt.push(String::from(string));
        }
        if is_password_correct(
            password,
            pw_hash_salt.get(0).unwrap(),
            pw_hash_salt.get(1).unwrap(),
        ) {
            return Some(userobj);
        }
        None
    }
}

fn is_password_correct(password: &str, password_hash: &str, salt: &str) -> bool {
    let mut hasher = Sha3_256::new();
    let password_with_salt = format!("{}_{}", password, salt);
    hasher.update(password_with_salt);
    let password_hash_result = hex::encode(hasher.finalize());
    if password_hash_result == password_hash {
        return true;
    }
    false
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::session::SessionService;
    use mongodb::{options::ClientOptions, Client};

    #[actix_rt::test]
    async fn test_service_user_login_ok() {
        let db = fetch_db().await;
        let user_service = UserService::new(&db);
        let login_positive = user_service.login("fg_10", "secretbird").await;
        assert_eq!(
            login_positive
                .unwrap()
                .get("username")
                .unwrap()
                .to_string()
                .replace('"', ""),
            String::from("fg_10")
        );
    }

    #[actix_rt::test]
    async fn test_service_session_ok() {
        let db = fetch_db().await;
        let user_service = UserService::new(&db);
        let session_service = SessionService::new(&db);

        let login_positive = user_service.login("fg_11", "secretbird").await;
        let session_object = session_service
            .create_session(login_positive.unwrap())
            .await;
        assert_ne!(&session_object, &"n.a.");

        let session_obj = session_service.validate_session(&session_object).await;

        assert!(session_obj.is_valid_session());
    }

    #[actix_rt::test]
    async fn test_service_login_nok() {
        let db = fetch_db().await;
        let user_service = UserService::new(&db);
        let session_service = SessionService::new(&db);
        let login_false = user_service.login("fg_10", "secret").await;
        assert_eq!(&login_false, &None);
        let session_object = session_service.validate_session("n.a.").await;
        assert!(!session_object.is_valid_session());
    }

    async fn fetch_db() -> Database {
        let client_options_future = ClientOptions::parse("mongodb://localhost:27017");
        let client_options = client_options_future.await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        client.database("nestbox_testing")
    }
}
