use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct Nestbox {
    uuid: String,
}

#[get("/nestboxes/{uuid}")]
pub async fn nestboxes_get(
    app_data: web::Data<crate::AppState>,
    nestbox: web::Path<Nestbox>,
) -> impl Responder {
    let result =
        web::block(move || app_data.service_container.user.get_by_uuid(&nestbox.uuid)).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
