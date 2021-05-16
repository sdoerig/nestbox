pub use crate::controller::utilities::{PagingQuery, Sanatiz};
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BreedReq {
    pub uuid: String,
}

#[get("/nestboxes/{uuid}/breeds")]
pub async fn breeds_get(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    breed_req: web::Path<BreedReq>,
    mut paging: web::Query<PagingQuery>,
) -> impl Responder {
    paging.sanatizing();
    let session_obj = app_data
        .service_container
        .session
        .validate_session(&req)
        .await;
    let breeds = app_data
        .service_container
        .breed
        .get_by_nestbox_uuid(&session_obj, &breed_req, &paging)
        .await;

    HttpResponse::Ok().json(breeds)
}
