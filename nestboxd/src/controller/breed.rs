pub use crate::controller::utilities::{PagingQuery, Sanatiz};
use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
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

#[derive(Deserialize)]
pub struct BirdReq {
    pub uuid: String,
    pub bird: String
}

#[post("/nestboxes/{uuid}/breeds")]
pub async fn breeds_post(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    breed_req: web::Json<BirdReq>,
) -> impl Responder {
    let session_obj = app_data
        .service_container
        .session
        .validate_session(&req)
        .await;
    if !session_obj.is_valid_session() {
        //User must have a valid session here, if not it does not make sense
        //to proceed.
        return HttpResponse::Unauthorized().json(())
    } 
    HttpResponse::Ok().json(())


}

