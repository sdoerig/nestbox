use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct BreedReq {
    uuid: String,
}


#[get("/nestboxes/{uuid}/breeds")]
pub async fn breeds_get(
    app_data: web::Data<crate::AppState>,
    nestbox: web::Path<BreedReq>,
) -> impl Responder {
    let nestbox_result = web::block(move || {
        app_data
            .service_container
            .nestbox
            .get_by_uuid(&nestbox.uuid)
    })
    .await;
    let nestbox = match nestbox_result {
        Ok(doc) => doc.unwrap(),
        Err(_e) => return HttpResponse::NotFound().finish(),
    };
    HttpResponse::NotFound().finish()
}
