use actix_web::{middleware::Logger, App, HttpServer};
use extract_argv::{extract_argv, parse_yaml};
use mongodb::{options::ClientOptions, Client, Database};
use service::bird::BirdService;
use service::breed::BreedService;
use service::geolocation::GeolocationService;
use service::image::ImageService;
use service::nestbox::NestboxService;
use service::session::SessionService;
use service::user::UserService;
mod controller;
mod extract_argv;
mod service;

//
pub struct ServiceContainer {
    image: ImageService,
    nestbox: NestboxService,
    user: UserService,
    session: SessionService,
    breed: BreedService,
    bird: BirdService,
    geolocation: GeolocationService,
}

impl ServiceContainer {
    pub fn new(db: Database, image_directory: String) -> Self {
        ServiceContainer {
            nestbox: NestboxService::new(&db),
            user: UserService::new(&db),
            session: SessionService::new(&db),
            breed: BreedService::new(&db),
            bird: BirdService::new(&db),
            geolocation: GeolocationService::new(&db),
            image: ImageService::new(image_directory),
        }
    }
}

pub struct AppState {
    service_container: ServiceContainer,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_struct = parse_yaml(extract_argv());
    let server_http_bind = format!(
        "{}:{}",
        &config_struct.httpserver_ip, &config_struct.httpserver_port
    );
    let client_options = ClientOptions::parse(&config_struct.mongodb_uri)
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&config_struct.mongodb_database);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        let service_container =
            ServiceContainer::new(db.clone(), config_struct.image_directory.clone());

        App::new()
            .data(AppState { service_container })
            .service(controller::nestbox::nestboxes_get)
            .service(controller::user::login_post)
            .service(controller::breed::breeds_get)
            .service(controller::bird::birds_get)
            .service(controller::breed::breeds_post)
            .service(controller::nestbox::nestboxes_locations_post)
            .service(controller::nestbox::nestboxes_images_post)
            .wrap(Logger::default())
    })
    .bind(server_http_bind)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::controller::{
        req_structs::LoginReq,
        res_structs::{LoginResponse, NestboxResponse},
        utilities::DocumentResponse,
        validator::is_uuid,
    };

    use super::*;

    use actix_web::{http::StatusCode, test, App};

    enum EndPoints {
        Birds,
        Geolocations,
        Breeds
    }

    #[actix_rt::test]
    async fn test_200_login_post_ok() {
        let uri = "/login";
        // {"username":"fg_199","password":"secretbird"}
        let user_name = String::from("fg_199");
        let user_data = LoginReq {
            username: user_name.clone(),
            password: String::from("secretbird"),
        };
        let svr_resp = build_login_post_app(uri, &user_data).await;
        assert_eq!(svr_resp.status(), StatusCode::OK);
        let response: LoginResponse = test::read_body_json(svr_resp).await;
        assert!(response.success);
        assert!(response.username == user_name);
        assert!(is_uuid(&response.session))
    }

    #[actix_rt::test]
    async fn test_401_login_post_nok() {
        let uri = "/login";
        // {"username":"fg_199","password":"secretbird"}
        let user_name = String::from("fg_199");
        let user_data = LoginReq {
            username: user_name.clone(),
            password: String::from("wronpass"),
        };
        let svr_resp = build_login_post_app(uri, &user_data).await;
        assert_eq!(svr_resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn test_200_nestbox_get() {
        let uri = "/nestboxes/9ede3c8c-f552-4f74-bb8c-0b574be9895c";
        let svr_resp = build_nest_box_app(uri).await;
        assert_eq!(svr_resp.status(), StatusCode::OK);
        let response: NestboxResponse = test::read_body_json(svr_resp).await;
        assert!(response.uuid == String::from("9ede3c8c-f552-4f74-bb8c-0b574be9895c"));
    }

    #[actix_rt::test]
    async fn test_404_nestbox_get() {
        let uri = "/nestboxes/9ede3c8c-eeee-ffff-aaaa-0b574be9895c";
        let svr_resp = build_nest_box_app(uri).await;
        assert_eq!(svr_resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_401_birds_get() {
        let uri = "/birds?page_limit=100&page_number=1";
        let svr_resp = build_paging_get_app(EndPoints::Birds ,uri, "").await;
        assert_eq!(svr_resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn test_200_bird_get_ok() {
        let uri = "/login";
        // {"username":"fg_199","password":"secretbird"}
        let user_name = String::from("fg_200");
        let user_data = LoginReq {
            username: user_name.clone(),
            password: String::from("secretbird"),
        };
        let svr_login_resp = build_login_post_app(uri, &user_data).await;
        let login_response: LoginResponse = test::read_body_json(svr_login_resp).await;
        let uri = "/birds?page_limit=100&page_number=1";
        let svr_resp = build_paging_get_app(EndPoints::Birds, uri, &login_response.session).await;
        assert_eq!(svr_resp.status(), StatusCode::OK);
        let mut paging_response: DocumentResponse = test::read_body_json(svr_resp).await;
        let total_documents = paging_response.counted_documents;
        let mut count_documents: i64 = 0;
        while !paging_response.documents.is_empty() {
            count_documents += paging_response.documents.len() as i64;
            let uri = format!(
                "/birds?page_limit=100&page_number={}",
                paging_response.page_number + 1
            );
            let svr_resp = build_paging_get_app(EndPoints::Birds, &uri, &login_response.session).await;
            paging_response = test::read_body_json(svr_resp).await;
        }
        assert!(total_documents == count_documents);
    }

    async fn build_paging_get_app(endpoint: EndPoints, uri: &str, sessiontoken: &str) -> actix_web::dev::ServiceResponse {
        let mut app = match endpoint {
            EndPoints::Birds => 
                test::init_service(
                    App::new()
                        .data(AppState {
                            service_container: ServiceContainer::new(get_db().await, String::from("/tmp/")),
                        })
                        .service(controller::bird::birds_get),
                )
                .await,
            EndPoints::Geolocations => 
                test::init_service(
                    App::new()
                        .data(AppState {
                            service_container: ServiceContainer::new(get_db().await, String::from("/tmp/")),
                        })
                        .service(controller::bird::birds_get),
                )
                .await,
            EndPoints::Breeds => 
                test::init_service(
                    App::new()
                        .data(AppState {
                            service_container: ServiceContainer::new(get_db().await, String::from("/tmp/")),
                        })
                        .service(controller::bird::birds_get),
                )
                .await,
        };
        
        if is_uuid(sessiontoken) {
            test::TestRequest::get()
                .uri(uri)
                .header("Authorization", format!("Basic {}", sessiontoken))
                .send_request(&mut app)
                .await
        } else {
            test::TestRequest::get()
                .uri(uri)
                .send_request(&mut app)
                .await
        }
    }

    async fn build_nest_box_app(uri: &str) -> actix_web::dev::ServiceResponse {
        let mut app = test::init_service(
            App::new()
                .data(AppState {
                    service_container: ServiceContainer::new(get_db().await, String::from("/tmp/")),
                })
                .service(controller::nestbox::nestboxes_get),
        )
        .await;
        let svr_resp = test::TestRequest::get()
            .uri(uri)
            .send_request(&mut app)
            .await;
        svr_resp
    }

    async fn build_login_post_app(
        uri: &str,
        user_data: &LoginReq,
    ) -> actix_web::dev::ServiceResponse {
        let mut app = test::init_service(
            App::new()
                .data(AppState {
                    service_container: ServiceContainer::new(get_db().await, String::from("/tmp/")),
                })
                .service(controller::user::login_post),
        )
        .await;
        //TestRequest::post().uri("/users").set_json
        let svr_resp = test::TestRequest::post()
            .uri(uri)
            .header("Content-Type", "application/json")
            .set_json(user_data)
            .send_request(&mut app)
            .await;
        svr_resp
    }

    async fn get_db() -> Database {
        let client_options_future = ClientOptions::parse("mongodb://localhost:27017");
        let client_options = client_options_future.await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        client.database("nestbox_testing")
    }
}
