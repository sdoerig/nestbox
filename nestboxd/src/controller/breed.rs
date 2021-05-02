use actix_web::{get, web, HttpResponse, Responder};
use bson::Document;
use serde::{Deserialize};
use crate::service::breed::BreedService;
#[derive(Deserialize)]
pub struct BreedReq {
    uuid: String,
}


#[get("/nestboxes/{uuid}/breeds")]
pub async fn breeds_get(
    app_data: web::Data<crate::AppState>,
    nestbox: web::Path<BreedReq>,
) -> impl Responder {
    let breed_col = BreedService::new(app_data.service_container.db.collection("breeds"));

    let nestbox_result =  web::block(move || {
        app_data
            .service_container
            .nestbox
            .get_by_uuid(&nestbox.uuid)
    })
    .await;
    let nestbox = match nestbox_result {
        Ok(doc) => doc.unwrap(),
        Err(_e) => return HttpResponse::NotFound().finish(),
    };

    let breeds = web::block(move || {breed_col.get_by_nestbox(&nestbox)}).await;
    let mut breeds_doc: Vec<Document> = Vec::new();
    let mut cursor = match breeds {
        Ok(c) => c,
        Err(_e) => return HttpResponse::NotFound().finish()
    };
    // cursor.collect::<Document>(); // how to fetch the Documents here?
    HttpResponse::Ok().json(breeds_doc)
}
