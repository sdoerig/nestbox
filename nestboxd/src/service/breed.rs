use bson::{doc, Document};

use futures::{executor::block_on, FutureExt, StreamExt};
use mongodb::{error::Error, Collection, Cursor};
use serde::Serialize;

#[derive(Clone)]
pub struct BreedService {
    collection: Collection,
}

#[derive(Serialize)]
pub struct DocumentResponse {
    pub documents: Vec<Document>,
    pub counted_documents: i64,
}

impl BreedService {
    pub fn new(collection: Collection) -> BreedService {
        BreedService { collection }
    }

    pub async fn get_by_nestbox_uuid(&self, nestbox_uuid: &str) -> DocumentResponse {
        //let mut results_doc: Vec<Document> = Vec::new();
        let res = self
            .collection
            .find(doc! {"nestbox_uuid": nestbox_uuid}, None);
        let blocked_res = block_on(res);
        let counted_documents_res = self.get_by_nestbox_count(nestbox_uuid).await;

        let mut documents: Vec<Document> = Vec::new();
        let result_documents = match blocked_res {
            Ok(c) => c.collect().await,
            Err(_e) => Vec::new(),
        };

        for r in result_documents {
            match r {
                Ok(d) => documents.push(d),
                Err(_e) => continue,
            }
        }
        let counted_documents = match counted_documents_res {
            Ok(i) => i,
            Err(_e) => 0,
        };

        DocumentResponse {
            documents,
            counted_documents,
        }
    }

    pub async fn get_by_nestbox_count(&self, nestbox_uuid: &str) -> Result<i64, Error> {
        self.collection
            .count_documents(doc! {"nestbox_uuid": nestbox_uuid}, None)
            .await
    }
}
