
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

const MAX_PAGE_LIMIT: i64 = 100;

#[derive(Deserialize)]
pub struct BreedReq {
    pub uuid: String,
}


#[derive(Deserialize)]
pub struct PagingQuery {
    pub page_limit: i64,
    pub page_number: i64,
}

trait Sanatiz {
    fn sanatizing(&mut self);
}

impl Sanatiz for PagingQuery {
    fn sanatizing(&mut self) {
        // range check page number page numbers start from one - so if one 
        if self.page_number - 1 < 1 {
            self.page_number = 1;
        }
        if self.page_limit > MAX_PAGE_LIMIT || self.page_limit <= 0 {
            self.page_limit = MAX_PAGE_LIMIT
        }
    }
}

#[get("/nestboxes/{uuid}/breeds")]
pub async fn breeds_get(
    app_data: web::Data<crate::AppState>,
    breed_req: web::Path<BreedReq>,
    mut paging: web::Query<PagingQuery>,
) -> impl Responder {
    paging.sanatizing();
    let breeds = app_data
        .service_container
        .breed
        .get_by_nestbox_uuid(&breed_req, &paging)
        .await;

    HttpResponse::Ok().json(breeds)
}
