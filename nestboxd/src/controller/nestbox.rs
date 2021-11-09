use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use bson::{Document, doc};
use chrono::format::StrftimeItems;
use serde::{Deserialize, Serialize};

use crate::controller::error_message::{BAD_REQUEST, INTERNAL_SERVER_ERROR};

use super::{
    error_message::{create_error_message, NOT_FOUND},
    req_structs::{GeolocationReq, NestboxReq},
    utilities::{nestbox_req_is_authorized, parse_auth_header},
    validator::Validator,
};

#[derive(Serialize, Deserialize)]
pub struct NestboxResponse {
    pub uuid: String,
}

impl NestboxResponse {
    pub fn new(doc: Document) -> Self {
        let mut uuid = String::from("");
        if let Some(b) = doc.get("uuid") {
            uuid = b.to_string().replace('"', "");
        }
        NestboxResponse { uuid }
    }
}

#[get("/nestboxes/{uuid}")]
pub async fn nestboxes_get(
    app_data: web::Data<crate::AppState>,
    nestbox: web::Path<NestboxReq>,
) -> HttpResponse {
    if !nestbox.is_valid() {
        return HttpResponse::BadRequest().json(create_error_message(BAD_REQUEST));
    }

    let result = app_data
        .service_container
        .nestbox
        .get_by_uuid(&nestbox.uuid)
        .await;
    match result {
        Ok(doc) => match doc {
            Some(d) => HttpResponse::Ok().json(NestboxResponse::new(d)),
            None => HttpResponse::NotFound().json(create_error_message(NOT_FOUND)),
        },
        Err(_e) => HttpResponse::NotFound().finish(),
    }
}

#[post("/nestboxes/{uuid}/images")]
pub async fn nestboxes_images_post(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    nestbox_req: web::Path<NestboxReq>,
    payload: Multipart,
) -> impl Responder {
    let session_uuid = parse_auth_header(&req);
    let session = app_data
        .service_container
        .session
        .validate_session(&session_uuid)
        .await;
    if !nestbox_req.is_valid() {
        return HttpResponse::BadRequest().json(create_error_message(BAD_REQUEST));
    }
    if let Some(value) = nestbox_req_is_authorized(&session, &app_data, &nestbox_req).await {
        return value;
    }
    let upload_status = app_data.service_container.image.save_file(payload).await;
    if let Some(file_name) = upload_status {
        if app_data
            .service_container
            .nestbox
            .append_image_by_uuid(&nestbox_req.uuid, &file_name)
            .await
        {
            return HttpResponse::Created().json(doc! {"file_name": file_name})
        } else {
            return HttpResponse::BadRequest().json(doc! {"file_name": "undefined"})
        }
    }

    HttpResponse::BadRequest().json(doc! {"file_name": "undefined"})
}

#[post("/nestboxes/{uuid}/geolocations")]
pub async fn nestboxes_locations_post(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    nestbox_req: web::Path<NestboxReq>,
    geoloc_req: web::Json<GeolocationReq>,
) -> impl Responder {
    let session_uuid = parse_auth_header(&req);
    let session = app_data
        .service_container
        .session
        .validate_session(&session_uuid)
        .await;
    if !nestbox_req.is_valid() {
        return HttpResponse::BadRequest().json(create_error_message(BAD_REQUEST));
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


