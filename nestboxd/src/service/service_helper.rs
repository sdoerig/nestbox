use bson::{ Document};
use futures::StreamExt;
use mongodb::{error::Error};

pub(crate) async fn read_mongodb_cursor(blocked_res: Result<mongodb::Cursor, Error>) -> Vec<Document> {
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
    documents
}