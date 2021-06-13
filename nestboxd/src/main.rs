use actix_web::{middleware::Logger, App, HttpServer};
use extract_argv::{extract_argv, parse_yaml};
use mongodb::{options::ClientOptions, Client, Database};
use service::breed::BreedService;
use service::geolocation::GeolocationService;
use service::nestbox::NestboxService;
use service::session::SessionService;
use service::user::UserService;
use service::bird::BirdService;

mod controller;
mod extract_argv;
mod service;

pub struct ServiceContainer {
    db: Database,
    nestbox: NestboxService,
    user: UserService,
    session: SessionService,
    breed: BreedService,
    bird: BirdService,
    geolocation: GeolocationService
}

impl ServiceContainer {
    pub fn new(db: Database) -> Self {
        let nestboxes_col = db.collection("nestboxes");
        let users_col = db.collection("users");
        let session_col = db.collection("sessions");
        let breed_col = db.collection("breeds");        
        let bird_col = db.collection("birds");
        let geolocation_col = db.collection("geolocations");
        ServiceContainer {
            db,
            nestbox: NestboxService::new(nestboxes_col),
            user: UserService::new(users_col),
            session: SessionService::new(session_col),
            breed: BreedService::new(breed_col),
            bird: BirdService::new(bird_col),
            geolocation: GeolocationService::new(geolocation_col)
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
        let service_container = ServiceContainer::new(db.clone());

        App::new()
            .data(AppState { service_container })
            .service(controller::nestbox::nestboxes_get)
            .service(controller::user::login_post)
            .service(controller::breed::breeds_get)
            .service(controller::bird::birds_get)
            .service(controller::breed::breeds_post)
            .service(controller::nestbox::nestboxes_locations_post)
            .wrap(Logger::default())
    })
    .bind(server_http_bind)?
    .run()
    .await
}
