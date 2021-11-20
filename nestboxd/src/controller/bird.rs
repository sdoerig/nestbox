use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::controller::{res_structs::MapDocument, utilities::DocumentResponse};

use super::{
    error_message::{create_error_message, UNAUTHORIZED},
    res_structs::BirdResponse,
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
    let (birds, counted_documents) = app_data
        .service_container
        .bird
        .get_by_mandant_uuid(&session_obj, &paging)
        .await;
    let mut bird_documents: Vec<BirdResponse> = Vec::new();
    for bird in birds {
        bird_documents.push(BirdResponse::map_doc(&bird));
    }

    HttpResponse::Ok().json(DocumentResponse::<BirdResponse>::new(
        bird_documents,
        counted_documents,
        &paging,
    ))
}
