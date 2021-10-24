use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use bson::doc;
use serde::{Deserialize, Serialize};

use crate::controller::error_message::{BAD_REQUEST, INTERNAL_SERVER_ERROR};

use super::{
    error_message::{create_error_message, NOT_FOUND},
    req_structs::{GeolocationReq, NestboxReq},
    utilities::{nestbox_req_is_authorized, parse_auth_header},
    validator::Validator
};

#[derive(Serialize, Deserialize)]
struct NestboxResponse {
    uuid: String,
}

#[get("/nestboxes/{uuid}")]
pub async fn nestboxes_get(
    app_data: web::Data<crate::AppState>,
    nestbox: web::Path<NestboxReq>,
) -> impl Responder {
    if !nestbox.is_valid() {
        return HttpResponse::BadRequest().json(create_error_message(BAD_REQUEST))
    }

    let result = app_data
        .service_container
        .nestbox
        .get_by_uuid(&nestbox.uuid)
        .await;
    match result {
        Ok(doc) => match doc {
            Some(d) => HttpResponse::Ok().json(d),
            None => HttpResponse::NotFound().json(create_error_message(NOT_FOUND)),
        },
        Err(_e) => HttpResponse::NotFound().finish(),
    }
}

#[post("/nestboxes/{uuid}/geolocations")]
pub async fn nestboxes_locations_post(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    nestbox_req: web::Path<NestboxReq>,
    geoloc_req: web::Json<GeolocationReq>,
) -> impl Responder {
    let session = app_data
        .service_container
        .session
        .validate_session(&parse_auth_header(&req))
        .await;
    if !nestbox_req.is_valid() {
        return HttpResponse::BadRequest().json(create_error_message(BAD_REQUEST))
    }

    if let Some(value) = nestbox_req_is_authorized(&session, &app_data, &nestbox_req).await {
        return value;
    }

    match app_data
        .service_container
        .geolocation
        .post_geolocation(&nestbox_req.uuid, geoloc_req.long, geoloc_req.lat)
        .await
    {
        crate::service::service_helper::InsertResult::Ok(d) => {
            HttpResponse::Created().json(doc! {"inserted_id": d })
        }
        crate::service::service_helper::InsertResult::TerminationError => {
            HttpResponse::InternalServerError().json(create_error_message(INTERNAL_SERVER_ERROR))
        }
        crate::service::service_helper::InsertResult::InsertError => {
            HttpResponse::InternalServerError().json(create_error_message(INTERNAL_SERVER_ERROR))
        }
    }
}
