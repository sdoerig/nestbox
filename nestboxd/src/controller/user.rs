use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::error_message::{create_error_message, UNAUTHORIZED};

#[derive(Deserialize, Serialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Serialize)]
pub struct LoginRes {
    pub username: String,
    pub success: bool,
    pub session: String,
}

#[post("/login")]
pub async fn login_post(
    app_data: web::Data<crate::AppState>,
    login: web::Json<LoginReq>,
) -> impl Responder {
    let user_response = app_data
        .service_container
        .user
        .login(&login.username, &login.password)
        .await;

    match user_response {
        Some(user_obj) => {
            return HttpResponse::Ok().json(LoginRes {
                username: login.username.clone(),
                success: true,
                session: app_data
                    .service_container
                    .session
                    .create_session(user_obj)
                    .await, //String::from("n.a."),
            })
        }

        None => {
            app_data
                .service_container
                .session
                .remove_session_by_username(&login.username)
                .await;
            HttpResponse::Unauthorized().json(create_error_message(UNAUTHORIZED))
        }
    }
}
