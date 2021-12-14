use mongodb::bson::{doc, Document};

use super::{
    res_structs::{BirdResponse, MapDocument},
    service_helper as sa,
};
use crate::controller::utilities::{PagingQuery, SessionObject};
use mongodb::{error::Error, Collection, Database};

const BIRDS: &str = "birds";

#[derive(Clone)]
pub struct BirdService {
    collection: Collection<Document>,
}

impl BirdService {
    pub fn new(db: &Database) -> Self {
        BirdService {
            collection: db.collection(BIRDS),
        }
    }

    pub async fn get_by_mandant_uuid(
        &self,
        session_obj: &SessionObject,
        paging: &PagingQuery,
    ) -> (Vec<BirdResponse>, i64) {
        let res = self
            .collection
            .aggregate(
                vec![
                    doc! {"$match": {"mandant_uuid": {"$eq": session_obj.get_mandant_uuid()}}},
                    doc! { "$sort" : { "bird" : 1} },
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

        let mut bird_documents: Vec<BirdResponse> = Vec::new();
        for bird in documents {
            bird_documents.push(BirdResponse::map_doc(&bird));
        }

        (bird_documents, counted_documents as i64)
    }

    pub async fn get_by_mandant_uuid_count(
        &self,
        session_obj: &SessionObject,
    ) -> Result<u64, Error> {
        self.collection
            .count_documents(doc! {"mandant_uuid": session_obj.get_mandant_uuid()}, None)
            .await
    }
}
