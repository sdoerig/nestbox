use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::{
    controller::utilities::DocumentResponse, service::res_structs::BirdResponse, ServiceContainer,
};

use super::{
    error_message::{create_error_message, UNAUTHORIZED},
    utilities::{parse_auth_header, PagingQuery, Sanatiz},
};

#[get("/birds")]
pub async fn birds_get(
    app_data: web::Data<ServiceContainer>,
    req: HttpRequest,
    mut paging: web::Query<PagingQuery>,
) -> impl Responder {
    paging.sanatizing();

    let session_obj = app_data
        .session
        .validate_session(&parse_auth_header(&req))
        .await;
    if !session_obj.is_valid_session() {
        return HttpResponse::Unauthorized().json(create_error_message(UNAUTHORIZED));
    }
    let (birds, counted_documents) = app_data
        .bird
        .get_by_mandant_uuid(&session_obj, &paging)
        .await;

    HttpResponse::Ok().json(DocumentResponse::<BirdResponse>::new(
        birds,
        counted_documents,
        &paging,
    ))
}
