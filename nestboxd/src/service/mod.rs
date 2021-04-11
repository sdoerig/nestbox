use bson::{doc, Document};
use futures::executor::block_on;
use mongodb::{error::Error, Collection};

use sha3::{Digest, Sha3_256};

use uuid::Uuid;

#[derive(Clone)]
pub struct NestboxService {
    collection: Collection,
}

impl NestboxService {
    pub fn new(collection: Collection) -> NestboxService {
        NestboxService { collection }
    }

    pub fn get(&self) -> Result<Option<Document>, Error> {
        let res = self.collection.find_one(doc! {}, None);
        block_on(res)
    }

    pub fn get_by_uuid(&self, uuid: &str) -> Result<Option<Document>, Error> {
        let res = self.collection.find_one(doc! {"uuid": uuid}, None);
        block_on(res)
    }
}

#[derive(Clone)]
pub struct UserService {
    collection: Collection,
}

impl UserService {
    pub fn new(collection: Collection) -> UserService {
        UserService { collection }
    }

    pub async fn login(&self, email: &str, password: &str) -> Option<String> {
        let user_res = self
            .collection
            .find_one(doc! {"email": email}, None)
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
            return Some(String::from("password_is_correct"));
        }
        Some(String::from("password_is_incorrect"))
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
