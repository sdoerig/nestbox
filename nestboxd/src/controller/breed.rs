pub use crate::controller::utilities::{PagingQuery, Sanatiz};
use crate::controller::{
    error_message::BAD_REQUEST, utilities::nestbox_req_is_authorized, validator::Validator,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use mongodb::bson::doc;

use super::{
    error_message::{create_error_message, INTERNAL_SERVER_ERROR},
    req_structs::{BirdReq, NestboxReq},
    utilities::parse_auth_header,
};

#[get("/nestboxes/{uuid}/breeds")]
pub async fn breeds_get(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    breed_req: web::Path<NestboxReq>,
    mut paging: web::Query<PagingQuery>,
) -> impl Responder {
    if !breed_req.is_valid() {
        return HttpResponse::BadRequest().json(create_error_message(BAD_REQUEST));
    }

    paging.sanatizing();
    let session_obj = app_data
        .service_container
        .session
        .validate_session(&parse_auth_header(&req))
        .await;
    let breeds = app_data
        .service_container
        .breed
        .get_by_nestbox_uuid(&session_obj, &breed_req, &paging)
        .await;

    HttpResponse::Ok().json(breeds)
}

#[post("/nestboxes/{uuid}/breeds")]
pub async fn breeds_post(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    nestbox_req: web::Path<NestboxReq>,
    bird_req: web::Json<BirdReq>,
) -> impl Responder {
    if !nestbox_req.is_valid() {
        return HttpResponse::BadRequest().json(create_error_message(BAD_REQUEST));
    }

    // To post a new breed which means the user has discovered a nest
    // in a birdhouse the user must be
    // - authenticated
    // - nestbox and transmitted bird must belong to the same mandant as the user does
    // - if the bird does not have a uuid it is considered to create a new bird for
    //   the users mandant.
    let session = app_data
        .service_container
        .session
        .validate_session(&parse_auth_header(&req))
        .await;
    if let Some(value) = nestbox_req_is_authorized(&session, &app_data, &nestbox_req).await {
        return value;
    }
    match app_data
        .service_container
        .breed
        .post_breed(&session, &nestbox_req, &bird_req)
        .await
    {
        Ok(d) => HttpResponse::Created().json(doc! {"inserted_id": d.inserted_id }),
        Err(_e) => {
            HttpResponse::InternalServerError().json(create_error_message(INTERNAL_SERVER_ERROR))
        }
    }
}
