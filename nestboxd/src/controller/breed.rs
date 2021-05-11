use crate::service::breed::{BreedService};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct BreedReq {
    pub uuid: String
}

#[derive(Deserialize)]
pub struct PagingQuery {
    pub page_limit: i64,
    pub page_number: i64
}



#[get("/nestboxes/{uuid}/breeds")]
pub async fn breeds_get(
    app_data: web::Data<crate::AppState>,
    breed_req: web::Path<BreedReq>, paging: web::Query<PagingQuery>
) -> impl Responder {


    let breeds = app_data.service_container.breed.get_by_nestbox_uuid(&breed_req, &paging).await;

    HttpResponse::Ok().json(breeds)
}


