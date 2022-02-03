use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use mongodb::bson::doc;

use crate::{
    controller::error_message::{BAD_REQUEST, INTERNAL_SERVER_ERROR},
    ServiceContainer,
};

use super::{
    error_message::create_error_message,
    req_structs::{GeolocationReq, NestboxReq},
    utilities::{nestbox_req_is_authorized, parse_auth_header},
    validator::Validator,
};

#[get("/nestboxes/{uuid}")]
pub async fn nestboxes_get(
    app_data: web::Data<ServiceContainer>,
    nestbox: web::Path<NestboxReq>,
) -> HttpResponse {
    if !nestbox.is_valid() {
        return HttpResponse::BadRequest().json(create_error_message(BAD_REQUEST));
    }

    let result = app_data.nestbox.get_by_uuid(&nestbox.uuid).await;
    match result.get(0) {
        Some(nestbox) => HttpResponse::Ok().json(nestbox),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/nestboxes/{uuid}/images")]
pub async fn nestboxes_images_post(
    app_data: web::Data<ServiceContainer>,
    req: HttpRequest,
    nestbox_req: web::Path<NestboxReq>,
    payload: Multipart,
) -> impl Responder {
    let session_uuid = parse_auth_header(&req);
    let session = app_data.session.validate_session(&session_uuid).await;
    if !nestbox_req.is_valid() {
        return HttpResponse::BadRequest().json(create_error_message(BAD_REQUEST));
    }
    if let Some(value) = nestbox_req_is_authorized(&session, &app_data, &nestbox_req).await {
        return value;
    }
    let upload_status = app_data.image.save_file(payload).await;
    if let Some(file_name) = upload_status {
        if app_data
            .nestbox
            .append_image_by_uuid(&nestbox_req.uuid, &file_name)
            .await
        {
            return HttpResponse::Created().json(doc! {"file_name": file_name});
        } else {
            return HttpResponse::BadRequest().json(doc! {"file_name": "undefined"});
        }
    }

    HttpResponse::BadRequest().json(doc! {"file_name": "undefined"})
}

#[post("/nestboxes/{uuid}/geolocations")]
pub async fn nestboxes_locations_post(
    app_data: web::Data<ServiceContainer>,
    req: HttpRequest,
    nestbox_req: web::Path<NestboxReq>,
    geoloc_req: web::Json<GeolocationReq>,
) -> impl Responder {
    let session_uuid = parse_auth_header(&req);
    let session = app_data.session.validate_session(&session_uuid).await;
    if !nestbox_req.is_valid() {
        return HttpResponse::BadRequest().json(create_error_message(BAD_REQUEST));
    }

    if let Some(value) = nestbox_req_is_authorized(&session, &app_data, &nestbox_req).await {
        return value;
    }

    match app_data
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
