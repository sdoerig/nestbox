use actix_web::web::Data;
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
        App::new()
            .app_data(Data::new(ServiceContainer::new(
                db.clone(),
                config_struct.image_directory.clone(),
            )))
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
        req_structs::{BirdReq, GeolocationReq, LoginReq},
        utilities::DocumentResponse,
        validator::is_uuid,
    };
    use crate::service::res_structs::{
        BirdResponse, BreedResponse, LoginResponse, NestboxResponse,
    };

    use super::*;

    use actix_http::header::HeaderValue;
    use actix_web::{http::StatusCode, test, App};

    #[derive(Clone)]
    enum HttpMethod {
        Post,
        Get,
    }

    #[derive(Clone)]
    enum EndPoints {
        Birds(HttpMethod),
        Geolocations(HttpMethod),
        Breeds(HttpMethod),
        Login(HttpMethod),
        Nestboxes(HttpMethod),
    }

    enum RequestData {
        Login(LoginReq),
        Bird(BirdReq),
        Geolocation(GeolocationReq),
        Empty,
    }

    const NESTBOX_EXISTING: &str = "9ede3c8c-f552-4f74-bb8c-0b574be9895c";
    const NESTBOX_NOT_EXISTING: &str = "9ede3c8c-eeee-ffff-aaaa-0b574be9895c";
    const USER: &str = "fg_199";
    const USER_BIRDS_TEST: &str = "fg_198";
    const PASSWORD_CORRECT: &str = "secretbird";
    const PASSWORD_WRONG: &str = "wrongbird";
    const IMAGE_DIRECTORY: &str = "/tmp/";
    const USER_STRANGER_BREED_POST: &str = "fg_1001";
    const USER_STRANGER_GEOLOCATION_POST: &str = "fg_1002";
    const USER_MANDANT_1: &str = "fg_200";
    const USER_MANDANT_1_GEOLOCATION: &str = "fg_180";
    const NESTBOX_MANDANT_1: &str = "45f149a2-b05a-4de8-a358-6e704eb6efca";
    const BIRD_MANDANT_1: &str = "ffbf3bf5-868e-437b-b0e8-cf19ce2a6ad2";

    #[actix_rt::test]
    async fn test_200_login_post_ok() {
        let uri = "/login";
        // {"username":USER,"password":PASSWORD_CORRECT}
        let user_name = String::from(USER);
        let user_data = LoginReq {
            username: user_name.clone(),
            password: String::from(PASSWORD_CORRECT),
        };
        let svr_resp = build_app(
            EndPoints::Login(HttpMethod::Post),
            uri,
            "",
            RequestData::Login(user_data),
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::OK);
        let response: LoginResponse = test::read_body_json(svr_resp).await;
        assert!(response.success);
        assert!(response.username == user_name);
        assert!(is_uuid(&response.session))
    }

    #[actix_rt::test]
    async fn test_401_login_post_nok() {
        let uri = "/login";
        // {"username":USER,"password":PASSWORD_CORRECT}
        let user_name = String::from(USER);
        let user_data = LoginReq {
            username: user_name.clone(),
            password: String::from(PASSWORD_WRONG),
        };
        let svr_resp = build_app(
            EndPoints::Login(HttpMethod::Post),
            uri,
            "",
            RequestData::Login(user_data),
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn test_200_nestbox_get() {
        let uri = format!("/nestboxes/{}", NESTBOX_EXISTING);
        let svr_resp = build_app(
            EndPoints::Nestboxes(HttpMethod::Get),
            &uri,
            "",
            RequestData::Empty,
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::OK);
        let response: NestboxResponse = test::read_body_json(svr_resp).await;
        assert!(response.uuid == NESTBOX_EXISTING);
    }

    #[actix_rt::test]
    async fn test_404_nestbox_get() {
        let uri = format!("/nestboxes/{}", NESTBOX_NOT_EXISTING);
        let svr_resp = build_app(
            EndPoints::Nestboxes(HttpMethod::Get),
            &uri,
            "",
            RequestData::Empty,
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_401_birds_get() {
        let uri = "/birds?page_limit=100&page_number=1";
        let svr_resp = build_app(
            EndPoints::Birds(HttpMethod::Get),
            uri,
            "",
            RequestData::Empty,
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn test_200_bird_get_ok() {
        let login_response = login_ok(USER_BIRDS_TEST).await;
        let uri = "/birds?page_limit=100&page_number=1";
        let svr_resp = build_app(
            EndPoints::Birds(HttpMethod::Get),
            uri,
            &(login_response.session),
            RequestData::Empty,
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::OK);
        let mut paging_response: DocumentResponse<BirdResponse> =
            test::read_body_json(svr_resp).await;
        let total_documents = paging_response.counted_documents;
        let mut count_documents: i64 = 0;
        while !paging_response.documents.is_empty() {
            count_documents += paging_response.documents.len() as i64;
            let uri = format!(
                "/birds?page_limit=100&page_number={}",
                paging_response.page_number + 1
            );
            let svr_resp = build_app(
                EndPoints::Birds(HttpMethod::Get),
                &uri,
                &login_response.session,
                RequestData::Empty,
            )
            .await;
            assert_eq!(
                svr_resp.status(),
                StatusCode::OK,
                "Failed at uri {}, token {}",
                uri,
                &login_response.session
            );
            paging_response = test::read_body_json(svr_resp).await;
        }
        assert!(total_documents == count_documents);
    }

    #[actix_rt::test]
    async fn test_200_breed_get_ok() {
        let uri = format!(
            "/nestboxes/{}/breeds?page_limit=3&page_number=1",
            NESTBOX_EXISTING
        );
        let svr_resp = build_app(
            EndPoints::Breeds(HttpMethod::Get),
            &uri,
            "",
            RequestData::Empty,
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::OK);
        let mut paging_response: DocumentResponse<BreedResponse> =
            test::read_body_json(svr_resp).await;
        let total_documents = paging_response.counted_documents;
        let mut count_documents: i64 = 0;
        while !paging_response.documents.is_empty() {
            count_documents += paging_response.documents.len() as i64;
            let uri = format!(
                "/nestboxes/{}/breeds?page_limit=3&page_number={}",
                NESTBOX_EXISTING,
                paging_response.page_number + 1
            );
            let svr_resp = build_app(
                EndPoints::Breeds(HttpMethod::Get),
                &uri,
                "",
                RequestData::Empty,
            )
            .await;
            paging_response = test::read_body_json(svr_resp).await;
        }
        assert!(total_documents == count_documents);
    }

    #[actix_rt::test]
    async fn test_204_breeds_post_ok() {
        // curl \
        //    -H "Authorization: Basic b955d5ab-531d-45a5-b610-5b456fa509d9" \
        //    --H "Content-Type: application/json" \
        //    --request POST \
        //    --data '{"bird_uuid": "a4152a25-b734-4748-8a43-2401ed387c65", "bird":"a"}' \
        //    http://127.0.0.1:8080/nestboxes/9973e59f-771d-452f-9a1b-8b4a6d5c4f95/breeds
        let uri = format!("/nestboxes/{}/breeds", NESTBOX_MANDANT_1);
        let login_response = login_ok(USER_MANDANT_1).await;
        // nestbox 45f149a2-b05a-4de8-a358-6e704eb6efca
        // bird "ffbf3bf5-868e-437b-b0e8-cf19ce2a6ad2",
        // "mandant_uuid" : "5bcb187b-996a-4169-8f12-cc315c2b22f7"
        let bird_data: BirdReq = BirdReq {
            bird: String::from("_"),
            bird_uuid: String::from(BIRD_MANDANT_1),
        };
        let svr_resp = build_app(
            EndPoints::Breeds(HttpMethod::Post),
            &uri,
            &login_response.session,
            RequestData::Bird(bird_data),
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::CREATED);
        let resp: BreedResponse = test::read_body_json(svr_resp).await;
        assert!(is_uuid(&resp.uuid));
        assert!(
            resp.bird_uuid == BIRD_MANDANT_1,
            "resp.bird_uuid {} expected {}",
            resp.bird_uuid,
            BIRD_MANDANT_1
        );
    }

    #[actix_rt::test]
    async fn test_204_geolocation_post_ok() {
        let uri = format!("/nestboxes/{}/geolocations", NESTBOX_MANDANT_1);
        let login_response = login_ok(USER_MANDANT_1_GEOLOCATION).await;
        let geolocation = GeolocationReq {
            long: 8.005,
            lat: 48.05,
        };
        let svr_resp = build_app(
            EndPoints::Geolocations(HttpMethod::Post),
            &uri,
            &login_response.session,
            RequestData::Geolocation(geolocation),
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::CREATED);
        //let resp: BreedResponse = test::read_body_json(svr_resp).await;
    }

    #[actix_rt::test]
    async fn test_401_geolocation_post_no_session_unauthorized() {
        let uri = format!("/nestboxes/{}/geolocations", NESTBOX_MANDANT_1);
        let geolocation = GeolocationReq {
            long: 8.005,
            lat: 48.05,
        };
        let svr_resp = build_app(
            EndPoints::Geolocations(HttpMethod::Post),
            &uri,
            "",
            RequestData::Geolocation(geolocation),
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn test_401_geolocation_post_authenticated_wrong_mandant() {
        let uri = format!("/nestboxes/{}/geolocations", NESTBOX_MANDANT_1);
        let login_response = login_ok(USER_STRANGER_GEOLOCATION_POST).await;
        let geolocation = GeolocationReq {
            long: 8.005,
            lat: 48.05,
        };
        let svr_resp = build_app(
            EndPoints::Geolocations(HttpMethod::Post),
            &uri,
            &login_response.session,
            RequestData::Geolocation(geolocation),
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::UNAUTHORIZED);
        //let resp: BreedResponse = test::read_body_json(svr_resp).await;
    }

    #[actix_rt::test]
    async fn test_401_breeds_authenticated_wrong_mandant() {
        let uri = format!("/nestboxes/{}/breeds", NESTBOX_MANDANT_1);
        let login_response = login_ok(USER_STRANGER_BREED_POST).await;
        let bird_data: BirdReq = BirdReq {
            bird: String::from("_"),
            bird_uuid: String::from(BIRD_MANDANT_1),
        };
        let svr_resp = build_app(
            EndPoints::Breeds(HttpMethod::Post),
            &uri,
            &login_response.session,
            RequestData::Bird(bird_data),
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn test_401_breeds_post_not_authorized() {
        let uri = format!("/nestboxes/{}/breeds", NESTBOX_MANDANT_1);
        let bird_data: BirdReq = BirdReq {
            bird: String::from("_"),
            bird_uuid: String::from(BIRD_MANDANT_1),
        };
        let svr_resp = build_app(
            EndPoints::Breeds(HttpMethod::Post),
            &uri,
            "9973e59f-771d-452f-9a1b-8b4a6d5c4f95",
            RequestData::Bird(bird_data),
        )
        .await;
        assert_eq!(svr_resp.status(), StatusCode::UNAUTHORIZED);
    }

    async fn login_ok(user: &str) -> LoginResponse {
        let uri = "/login";
        let user_name = String::from(user);
        let user_data = LoginReq {
            username: user_name.clone(),
            password: String::from(PASSWORD_CORRECT),
        };
        let svr_login_resp = build_app(
            EndPoints::Login(HttpMethod::Post),
            uri,
            "",
            RequestData::Login(user_data),
        )
        .await;
        let login_response: LoginResponse = test::read_body_json(svr_login_resp).await;
        login_response
    }

    async fn build_app(
        endpoint: EndPoints,
        uri: &str,
        sessiontoken: &str,
        req: RequestData,
    ) -> actix_web::dev::ServiceResponse {
        let mut http_method = HttpMethod::Get;
        let app = match endpoint {
            EndPoints::Birds(m) => {
                http_method = m.clone();
                test::init_service(
                    App::new()
                        .app_data(Data::new(ServiceContainer::new(
                            get_db().await,
                            String::from(IMAGE_DIRECTORY),
                        )))
                        .service(controller::bird::birds_get),
                )
                .await
            }
            EndPoints::Geolocations(m) => {
                http_method = m.clone();
                test::init_service(
                    App::new()
                        .app_data(Data::new(ServiceContainer::new(
                            get_db().await,
                            String::from(IMAGE_DIRECTORY),
                        )))
                        .service(controller::nestbox::nestboxes_locations_post),
                )
                .await
            }
            EndPoints::Breeds(m) => match m {
                HttpMethod::Post => {
                    http_method = m.clone();
                    test::init_service(
                        App::new()
                            .app_data(Data::new(ServiceContainer::new(
                                get_db().await,
                                String::from(IMAGE_DIRECTORY),
                            )))
                            .service(controller::breed::breeds_post),
                    )
                    .await
                }
                HttpMethod::Get => {
                    http_method = m.clone();
                    test::init_service(
                        App::new()
                            .app_data(Data::new(ServiceContainer::new(
                                get_db().await,
                                String::from(IMAGE_DIRECTORY),
                            )))
                            .service(controller::breed::breeds_get),
                    )
                    .await
                }
            },
            EndPoints::Login(m) => {
                // Caution POST only implemented.
                http_method = m.clone();
                test::init_service(
                    App::new()
                        .app_data(Data::new(ServiceContainer::new(
                            get_db().await,
                            String::from(IMAGE_DIRECTORY),
                        )))
                        .service(controller::user::login_post),
                )
                .await
            }
            EndPoints::Nestboxes(_) => {
                // Caution GET only implemented.
                test::init_service(
                    App::new()
                        .app_data(Data::new(ServiceContainer::new(
                            get_db().await,
                            String::from(IMAGE_DIRECTORY),
                        )))
                        .service(controller::nestbox::nestboxes_get),
                )
                .await
            }
        };

        let auth_token = format!("Basic {}", sessiontoken);
        match http_method {
            HttpMethod::Post => match req {
                RequestData::Empty => {
                    test::TestRequest::post()
                        .uri(uri)
                        .append_header((
                            actix_web::http::header::CONTENT_TYPE,
                            HeaderValue::from_static("application/json"),
                        ))
                        .append_header((
                            actix_web::http::header::AUTHORIZATION,
                            HeaderValue::from_str(&auth_token).unwrap(),
                        ))
                        .send_request(&app)
                        .await
                }
                RequestData::Login(req) => {
                    test::TestRequest::post()
                        .uri(uri)
                        .append_header((
                            actix_web::http::header::CONTENT_TYPE,
                            HeaderValue::from_static("application/json"),
                        ))
                        .append_header((
                            actix_web::http::header::AUTHORIZATION,
                            HeaderValue::from_str(&auth_token).unwrap(),
                        ))
                        .set_json(&req)
                        .send_request(&app)
                        .await
                }
                RequestData::Bird(req) => {
                    test::TestRequest::post()
                        .uri(uri)
                        .append_header((
                            actix_web::http::header::CONTENT_TYPE,
                            HeaderValue::from_static("application/json"),
                        ))
                        .append_header((
                            actix_web::http::header::AUTHORIZATION,
                            HeaderValue::from_str(&auth_token).unwrap(),
                        ))
                        .set_json(&req)
                        .send_request(&app)
                        .await
                }
                RequestData::Geolocation(req) => {
                    test::TestRequest::post()
                        .uri(uri)
                        .insert_header((
                            actix_web::http::header::CONTENT_TYPE,
                            HeaderValue::from_static("application/json"),
                        ))
                        .append_header((
                            actix_web::http::header::AUTHORIZATION,
                            HeaderValue::from_str(&auth_token).unwrap(),
                        ))
                        .set_json(&req)
                        .send_request(&app)
                        .await
                }
            },
            HttpMethod::Get => {
                test::TestRequest::get()
                    .uri(uri)
                    .insert_header((
                        actix_web::http::header::AUTHORIZATION,
                        HeaderValue::from_str(&auth_token).unwrap(),
                    ))
                    .send_request(&app)
                    .await
            }
        }
    }

    async fn get_db() -> Database {
        let client_options_future = ClientOptions::parse("mongodb://localhost:27017");
        let client_options = client_options_future.await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        client.database("nestbox_testing")
    }
}
