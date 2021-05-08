use crate::service::breed::{BreedService};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct BreedReq {
    pub uuid: String,
}

#[get("/nestboxes/{uuid}/breeds")]
pub async fn breeds_get(
    app_data: web::Data<crate::AppState>,
    breed_req: web::Path<BreedReq>,
) -> impl Responder {


    let breeds = app_data.service_container.breed.get_by_nestbox_uuid(&breed_req.uuid).await;

    HttpResponse::Ok().json(breeds)
}
