pub use crate::controller::utilities::{PagingQuery, Sanatiz};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use bson::doc;

use super::{
    error_message::{create_error_message, NESTBOX_OF_OTHER_MANDANT},
    req_structs::{BirdReq, NestboxReq},
};

#[get("/nestboxes/{uuid}/breeds")]
pub async fn breeds_get(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    breed_req: web::Path<NestboxReq>,
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

#[post("/nestboxes/{uuid}/breeds")]
pub async fn breeds_post(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    nestbox_req: web::Path<NestboxReq>,
    bird_req: web::Json<BirdReq>,
) -> impl Responder {
    // To post a new breed which means the user has discovered a nest
    // in a birdhouse the user must be
    // - authenticated
    // - nestbox and transmitted bird must belong to the same mandant as the user does
    // - if the bird does not have a uuid it is considered to create a new bird for
    //   the users mandant.
    let session = app_data
        .service_container
        .session
        .validate_session(&req)
        .await;
    if !session.is_valid_session() {
        //User must have a valid session here, if not it does not make sense
        //to proceed.
        return HttpResponse::Unauthorized().json(());
    }
    // check if user is allowed to post a breed on this specific nestbox...
    match app_data
        .service_container
        .nestbox
        .get_by_mandant_uuid(&session, &nestbox_req)
        .await
    {
        Ok(o) => match o {
            Some(_) => {}
            None => {
                // ... seems to be a nestbox of another mandant
                return HttpResponse::Unauthorized()
                    .json(create_error_message(NESTBOX_OF_OTHER_MANDANT))
            }
        },
        Err(_) => return HttpResponse::InternalServerError().json(()), 
    }

    app_data
        .service_container
        .breed
        .post_breed(&session, &nestbox_req, &bird_req)
        .await;
    HttpResponse::Ok().json(())
}
