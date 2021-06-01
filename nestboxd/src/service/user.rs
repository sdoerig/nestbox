use bson::{doc, Document};
use mongodb::{Collection};

use sha3::{Digest, Sha3_256};


#[derive(Clone)]
pub struct UserService {
    collection: Collection,
}

impl UserService {
    pub fn new(collection: Collection) -> UserService {
        UserService { collection }
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
            &pw_hash_salt.get(0).unwrap(),
            &pw_hash_salt.get(1).unwrap(),
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
    use futures::executor::block_on;
    use mongodb::{Client, options::ClientOptions};

    use super::*;

    #[test]
    fn test_login() {
        let client_options_future = ClientOptions::parse("mongodb://localhost:27017");
        let client_options = block_on(client_options_future).unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("nestbox");
        let users_col = db.collection("users");
        let user_service = UserService::new(users_col);
        let login_positive_future = user_service.login("fg_10", "secretbird");
        let login_positive = block_on(login_positive_future);
    }
}