pub use crate::controller::utilities::{PagingQuery, Sanatiz};
use crate::controller::{
    error_message::BAD_REQUEST,
    res_structs::MapDocument,
    utilities::{nestbox_req_is_authorized, DocumentResponse},
    validator::Validator,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use super::{
    error_message::{create_error_message, INTERNAL_SERVER_ERROR},
    req_structs::{BirdReq, NestboxReq},
    res_structs::BreedResponse,
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
    let (breeds, counted_documents) = app_data
        .service_container
        .breed
        .get_by_nestbox_uuid(&session_obj, &breed_req, &paging)
        .await;
    let mut breed_responses: Vec<BreedResponse> = Vec::new();
    for d in breeds {
        breed_responses.push(BreedResponse::map_doc(&d));
    }
    HttpResponse::Ok().json(DocumentResponse::<BreedResponse>::new(
        breed_responses,
        counted_documents,
        &paging,
    ))
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
        Ok(d) => HttpResponse::Created().json(BreedResponse::map_doc(&d)),
        Err(_e) => {
            HttpResponse::InternalServerError().json(create_error_message(INTERNAL_SERVER_ERROR))
        }
    }
}
