use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    controller::req_structs::LoginReq, service::res_structs::LoginResponse, ServiceContainer,
};

use super::error_message::{create_error_message, UNAUTHORIZED};

#[post("/login")]
pub async fn login_post(
    app_data: web::Data<ServiceContainer>,
    login: web::Json<LoginReq>,
) -> impl Responder {
    let user_response = app_data.user.login(&login.username, &login.password).await;

    match user_response {
        Some(user_obj) => {
            return HttpResponse::Ok().json(LoginResponse {
                username: login.username.clone(),
                success: true,
                session: app_data.session.create_session(user_obj).await, //String::from("n.a."),
            });
        }

        None => {
            app_data
                .session
                .remove_session_by_username(&login.username)
                .await;
            HttpResponse::Unauthorized().json(create_error_message(UNAUTHORIZED))
        }
    }
}
