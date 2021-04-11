use bson::{doc, Document};
use futures::executor::block_on;
use mongodb::{error::Error, Collection};


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
        let user_res = self.collection.find_one(doc! {"email": email}, None).await.ok()?;
        //block_on(user_res);
        let userobj = match user_res {
            Some(u) => u,
            None => return None
        };
        let password_bson = userobj.get("password_hash");
        let password_hash = match password_bson {
            Some(b) => b.as_str()?,
            None => return None
        };
        Some(String::from(password_hash))
    }
}
