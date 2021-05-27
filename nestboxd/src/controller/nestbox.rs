use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::{error_message::{NOT_FOUND, create_error_message}, req_structs::NestboxReq};

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
