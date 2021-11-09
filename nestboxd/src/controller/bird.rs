use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use super::{
    error_message::{create_error_message, UNAUTHORIZED},
    utilities::{parse_auth_header, PagingQuery, Sanatiz},
};

#[get("/birds")]
pub async fn birds_get(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
    mut paging: web::Query<PagingQuery>,
) -> impl Responder {
    paging.sanatizing();

    let session_obj = app_data
        .service_container
        .session
        .validate_session(&parse_auth_header(&req))
        .await;
    if !session_obj.is_valid_session() {
        return HttpResponse::Unauthorized().json(create_error_message(UNAUTHORIZED));
    }
    let birds = app_data
        .service_container
        .bird
        .get_by_mandant_uuid(&session_obj, &paging)
        .await;
    HttpResponse::Ok().json(birds)
}
