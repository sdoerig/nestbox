use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Serialize)]
pub struct LoginRes {
    pub email: String,
    pub password: String,
    pub badword: String,
}

#[post("/login")]
pub async fn login_post(
    app_data: web::Data<crate::AppState>,
    login: web::Json<LoginReq>,
) -> impl Responder {
    let password_hash = app_data
        .service_container
        .user
        .login(&login.email, &login.password)
        .await;
    match password_hash {
        Some(r) => {
            return HttpResponse::Ok().json(LoginRes {
                email: login.email.clone(),
                password: login.password.clone(),
                badword: r,
            })
        }
        None => {
            return HttpResponse::Ok().json(LoginRes {
                email: login.email.clone(),
                password: login.password.clone(),
                badword: String::from("jackass"),
            })
        }
    };
}
