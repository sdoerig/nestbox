use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
use bson::doc;
use serde::{Deserialize, Serialize};

use super::{error_message::{NOT_FOUND, create_error_message}, req_structs::NestboxReq, utilities::{nestbox_req_is_authorized, parse_auth_header}};

#[derive(Serialize, Deserialize)]
struct NestboxResponse {
    uuid: String,
}

#[get("/nestboxes/{uuid}")]
pub async fn nestboxes_get(
    app_data: web::Data<crate::AppState>,
    nestbox: web::Path<NestboxReq>,
) -> impl Responder {
    let result = app_data
        .service_container
        .nestbox
        .get_by_uuid(&nestbox.uuid)
        .await;
    match result {
        Ok(doc) => match doc {
            Some(d) => HttpResponse::Ok().json(d),
            None => HttpResponse::NotFound().json(create_error_message(NOT_FOUND)),
        },
        Err(_e) => HttpResponse::NotFound().finish(),
    }
}

#[post("/nestboxes/{uuid}/geolocations")]
pub async fn nestboxes_locations_post(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    nestbox_req: web::Path<NestboxReq>
) -> impl Responder {
    let session = app_data
        .service_container
        .session
        .validate_session(&parse_auth_header(&req))
        .await;
    if let Some(value) = nestbox_req_is_authorized(&session, &app_data, &nestbox_req).await {
        return value;
    }

    HttpResponse::Ok().json(doc! {"hello": "world"})
}
