use bson::{doc, Document};

use crate::controller::breed::{BreedReq, PagingQuery};
use futures::{executor::block_on, StreamExt};
use mongodb::{error::Error, options::FindOptions, Collection};
use serde::Serialize;

#[derive(Clone)]
pub struct BreedService {
    collection: Collection,
}

#[derive(Serialize)]
pub struct DocumentResponse {
    pub documents: Vec<Document>,
    pub counted_documents: i64,
    pub pages: i64,
    pub page_number: i64,
    pub page_limit: i64,
}

impl DocumentResponse {
    pub fn new(documents: Vec<Document>, counted_documents: i64, paging: &PagingQuery) -> Self {
        let pages = if counted_documents % paging.page_limit > 0 {
            counted_documents / paging.page_limit + 1
        } else {
            counted_documents / paging.page_limit
        };
        DocumentResponse {
            documents,
            counted_documents,
            pages,
            page_number: paging.page_number,
            page_limit: paging.page_limit,
        }
    }
}

impl BreedService {
    pub fn new(collection: Collection) -> BreedService {
        BreedService { collection }
    }

    pub async fn get_by_nestbox_uuid(
        &self,
        req: &BreedReq,
        paging: &PagingQuery,
    ) -> DocumentResponse {
        //let mut results_doc: Vec<Document> = Vec::new();
        let find_options = FindOptions::builder()
            .limit(Some(paging.page_limit))
            .skip(Some(paging.page_limit * (paging.page_number - 1)))
            .build();
        let res = self
            .collection
            .find(doc! {"nestbox_uuid": &req.uuid}, find_options);
        let blocked_res = block_on(res);
        let counted_documents_res = self.get_by_nestbox_count(&req.uuid).await;

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

        DocumentResponse::new(documents, counted_documents, paging)
    }

    pub async fn get_by_nestbox_count(&self, nestbox_uuid: &str) -> Result<i64, Error> {
        self.collection
            .count_documents(doc! {"nestbox_uuid": nestbox_uuid}, None)
            .await
    }
}
