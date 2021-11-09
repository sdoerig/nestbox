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
    use crate::controller::res_structs::NestboxResponse;

    use super::*;

    use actix_web::{http::StatusCode, test, App};

    #[actix_rt::test]
    async fn test_nestbox_get() {
        let mut app = test::init_service(
            App::new()
                .data(AppState {
                    service_container: ServiceContainer::new(get_db().await, String::from("/tmp/")),
                })
                .service(controller::nestbox::nestboxes_get),
        )
        .await;
        let svr_resp = test::TestRequest::get()
            .uri("/nestboxes/9ede3c8c-f552-4f74-bb8c-0b574be9895c")
            .send_request(&mut app)
            .await;
        assert_eq!(svr_resp.status(), StatusCode::OK);
        //let req =  test::TestRequest::get().uri("/nestboxes/9ede3c8c-f552-4f74-bb8c-0b574be9895c").to_request();
        let response: NestboxResponse = test::read_body_json(svr_resp).await;
        assert!(response.uuid == String::from("9ede3c8c-f552-4f74-bb8c-0b574be9895c"));
    }

    async fn get_db() -> Database {
        let client_options_future = ClientOptions::parse("mongodb://localhost:27017");
        let client_options = client_options_future.await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        client.database("nestbox_testing")
    }
}
