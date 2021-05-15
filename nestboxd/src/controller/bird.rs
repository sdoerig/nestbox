use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::utilities::{PagingQuery, Sanatiz};

const HTTP_AUTHORIZATION: &str = "Authorization";

#[get("/birds")]
pub async fn birds_get(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    mut paging: web::Query<PagingQuery>,
) -> impl Responder {
    paging.sanatizing();
    let session_token = match req.headers().get(HTTP_AUTHORIZATION) {
        Some(t) => t.to_str(),
        None => Ok("n.a"),
    };
    let session_obj = app_data
        .service_container
        .session
        .validate_session(session_token.unwrap())
        .await;

    HttpResponse::Ok().json(session_obj.get_mandant_uuid())
}
