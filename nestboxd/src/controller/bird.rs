use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use super::utilities::{PagingQuery, Sanatiz};

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
        .validate_session(&req)
        .await;
    let birds = app_data
        .service_container
        .bird
        .get_by_mandant_uuid(&session_obj, &paging)
        .await;
    HttpResponse::Ok().json(birds)
}
