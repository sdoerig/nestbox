use actix_web::{get, post, web, HttpResponse, Responder};
use futures::executor::block_on;
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct NestboxReq {
    uuid: String,
}

#[derive(Serialize, Deserialize)]
struct NestboxResponse {
    uuid: String
}


#[get("/nestboxes/{uuid}")]
pub async fn nestboxes_get(
    app_data: web::Data<crate::AppState>,
    nestbox: web::Path<NestboxReq>,
) -> impl Responder {
    let result =
        web::block(move || app_data.service_container.nestbox.get_by_uuid(&nestbox.uuid)).await;
    match result {
        Ok(doc) => HttpResponse::Ok().json(doc),
        Err(_e) => {
            HttpResponse::NotFound().finish()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String
}
#[derive(Deserialize, Serialize)]
pub struct LoginRes {
    pub email: String,
    pub password: String,
    pub badword: String
}

#[post("/login")]
pub async fn login_post(app_data: web::Data<crate::AppState>, 
    login: web::Json<LoginReq>) -> impl Responder {
    let password_hash = app_data.service_container.user.login(&login.email, &login.password).await;
    match password_hash {
        Some(r) => return HttpResponse::Ok().json(LoginRes{ email: login.email.clone(), 
            password: login.password.clone(), badword: r}),
        None => return HttpResponse::Ok().json(LoginRes{ email: login.email.clone(), 
            password: login.password.clone(), badword: String::from("jackass")})
    };
    

}
