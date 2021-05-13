use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
pub use crate::controller::utilities::{PagingQuery, Sanatiz};

#[derive(Deserialize)]
pub struct BreedReq {
    pub uuid: String,
}



#[get("/nestboxes/{uuid}/breeds")]
pub async fn breeds_get(
    app_data: web::Data<crate::AppState>,
    breed_req: web::Path<BreedReq>,
    mut paging: web::Query<PagingQuery>,
) -> impl Responder {
    paging.sanatizing();
    let breeds = app_data
        .service_container
        .breed
        .get_by_nestbox_uuid(&breed_req, &paging)
        .await;

    HttpResponse::Ok().json(breeds)
}


