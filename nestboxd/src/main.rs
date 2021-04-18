use actix_web::{App, HttpServer};
use extract_argv::{extract_argv, parse_yaml};
use mongodb::{options::ClientOptions, Client, Database};
use service::nestbox::NestboxService;
use service::user::UserService;

mod controller;
mod extract_argv;
mod service;

pub struct ServiceContainer {
    db: Database,
    nestbox: NestboxService,
    user: UserService,
}

impl ServiceContainer {
    pub fn new(db: Database) -> Self {
        let nestboxes_col = db.collection("nestboxes");
        let users_col = db.collection("users");

        ServiceContainer {
            db,
            nestbox: NestboxService::new(nestboxes_col),
            user: UserService::new(users_col),
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

    HttpServer::new(move || {
        let service_container = ServiceContainer::new(db.clone());

        App::new()
            .data(AppState { service_container })
            .service(controller::nestbox::nestboxes_get)
            .service(controller::user::login_post)
    })
    .bind(server_http_bind)?
    .run()
    .await
}
