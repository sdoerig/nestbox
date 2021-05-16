use bson::Document;
use mongodb::{error::Error, Collection};
use serde::Deserialize;

use serde::Serialize;

const MAX_PAGE_LIMIT: i64 = 100;

#[derive(Deserialize)]
pub struct PagingQuery {
    pub page_limit: i64,
    pub page_number: i64,
}

pub trait Sanatiz {
    fn sanatizing(&mut self);
}

impl Sanatiz for PagingQuery {
    fn sanatizing(&mut self) {
        // range check page number page numbers start from one - so if one
        if self.page_number - 1 < 1 {
            self.page_number = 1;
        }
        if self.page_limit > MAX_PAGE_LIMIT || self.page_limit <= 0 {
            self.page_limit = MAX_PAGE_LIMIT
        }
    }
}

pub struct SessionObject {
    valid_session: bool,
    session_key: String,
    mandant_uuid: String,
}

impl SessionObject {
    pub fn new(user_obj: Result<Option<Document>, Error>) -> Self {
        let session_document = match user_obj {
            Ok(od) => match od {
                Some(d) => d,
                None => Document::new(),
            },
            Err(_e) => Document::new(),
        };
        if session_document.is_empty() {
            // invalid session - so return - does not make sense to go any further...
            return SessionObject {
                valid_session: false,
                session_key: String::from("n.a."),
                mandant_uuid: String::from("n.a."),
            };
        }

        let (valid_mandant, mandant_uuid) = match session_document.get("mandant_uuid") {
            Some(b) => (true, b.to_string().replace('"', &"")),
            None => (false, String::from("n.a.")),
        };
        let (valid_session_key, session_key) = match session_document.get("session_key") {
            Some(b) => (true, b.to_string().replace('"', &"")),
            None => (false, String::from("n.a.")),
        };

        SessionObject {
            valid_session: (valid_mandant && valid_session_key),
            session_key,
            mandant_uuid,
        }
    }

    pub fn get_mandant_uuid(&self) -> &str {
        self.mandant_uuid.as_str()
    }

    pub fn get_session_key(&self) -> &str {
        self.session_key.as_str()
    }

    pub fn is_valid_session(&self) -> bool {
        self.valid_session
    }

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
