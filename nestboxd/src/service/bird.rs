use bson::doc;

use super::service_helper as sa;
use crate::controller::utilities::{DocumentResponse, PagingQuery, SessionObject};
use mongodb::{error::Error, Collection};

#[derive(Clone)]
pub struct BirdService {
    collection: Collection,
}

impl BirdService {
    pub fn new(collection: Collection) -> Self {
        BirdService { collection }
    }

    pub async fn get_by_mandant_uuid(
        &self,
        session_obj: &SessionObject,
        paging: &PagingQuery,
    ) -> DocumentResponse {
        let res = self
            .collection
            .aggregate(
                vec![
                    doc! {"$match": {"mandant_uuid": {"$eq": session_obj.get_mandant_uuid()}}},
                    doc! {"$skip": (paging.page_limit * (paging.page_number -1))},
                    doc! {"$limit": paging.page_limit},
                    doc! {"$project": {"_id": 0, "mandant_uuid": 0}},
                ],
                None,
            )
            .await;
        let counted_documents_res = self.get_by_mandant_uuid_count(session_obj).await;

        let documents = sa::read_mongodb_cursor(res).await;
        let counted_documents = match counted_documents_res {
            Ok(i) => i,
            Err(_e) => 0,
        };

        DocumentResponse::new(documents, counted_documents, paging)
    }

    pub async fn get_by_mandant_uuid_count(
        &self,
        session_obj: &SessionObject,
    ) -> Result<i64, Error> {
        self.collection
            .count_documents(doc! {"mandant_uuid": session_obj.get_mandant_uuid()}, None)
            .await
    }

    pub async fn get_by_uuid_and_mandant_uuid() {
        
    }
}
