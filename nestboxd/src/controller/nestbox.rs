use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct NestboxReq {
    uuid: String,
}

#[derive(Serialize, Deserialize)]
struct NestboxResponse {
    uuid: String,
}

#[get("/nestboxes/{uuid}")]
pub async fn nestboxes_get(
    app_data: web::Data<crate::AppState>,
    nestbox: web::Path<NestboxReq>,
) -> impl Responder {
    let result = web::block(move || {
        app_data
            .service_container
            .nestbox
            .get_by_uuid(&nestbox.uuid)
    })
    .await;
    match result {
        Ok(doc) => HttpResponse::Ok().json(doc),
        Err(_e) => HttpResponse::NotFound().finish(),
    }
}
