use actix_web::{web, HttpRequest, HttpResponse};
use bson::Document;
use mongodb::{error::Error};
use serde::Deserialize;

use serde::Serialize;

use super::error_message::UNAUTHORIZED;
use super::error_message::{NESTBOX_OF_OTHER_MANDANT, create_error_message};
use super::req_structs::NestboxReq;

const MAX_PAGE_LIMIT: i64 = 100;
const HTTP_AUTHORIZATION: &str = "Authorization";

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
    user_uuid: String,
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
                user_uuid: String::from("n.a."),
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
        let (valid_user_uuid, user_uuid) = match session_document.get("uuid") {
            Some(b) => (true, b.to_string().replace('"', &"")),
            None => (false, String::from("n.a.")),
        };

        SessionObject {
            user_uuid,
            valid_session: (valid_mandant && valid_session_key && valid_user_uuid),
            session_key,
            mandant_uuid,
        }
    }

    pub fn get_mandant_uuid(&self) -> &str {
        self.mandant_uuid.as_str()
    }

    pub fn get_user_uuid(&self) -> &str {
        self.user_uuid.as_str()
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

pub fn parse_auth_header(http_req: &HttpRequest) -> String {
    let session_token = match http_req.headers().get(HTTP_AUTHORIZATION) {
        Some(t) => t.to_str(),
        None => Ok("n.a."),
    };
    session_token.unwrap().replace("Basic ", "")
}

pub async fn nestbox_req_is_authorized(
    session: &super::utilities::SessionObject,
    app_data: &web::Data<crate::AppState>,
    nestbox_req: &web::Path<NestboxReq>,
) -> Option<HttpResponse> {
    if !session.is_valid_session() {
        //User must have a valid session here, if not it does not make sense
        //to proceed.
        return Some(HttpResponse::Unauthorized().json(create_error_message(UNAUTHORIZED)));
    }
    match app_data
        .service_container
        .nestbox
        .get_by_mandant_uuid(session, nestbox_req)
        .await
    {
        Ok(o) => match o {
            Some(_d) => None,
            None => {
                // ... seems to be a nestbox of another mandant
                Some(
                    HttpResponse::Unauthorized()
                        .json(create_error_message(NESTBOX_OF_OTHER_MANDANT)),
                )
            }
        },
        Err(_) => Some(HttpResponse::InternalServerError().json(())),
    }
}

