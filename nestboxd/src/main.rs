use actix_web::{App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use service::NestboxService;

mod controller;
mod service;

pub struct ServiceContainer {
  user: NestboxService,
}

impl ServiceContainer {
  pub fn new(user: NestboxService) -> Self {
    ServiceContainer { user }
  }
}

pub struct AppState {
  service_container: ServiceContainer,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
  let client = Client::with_options(client_options).unwrap();
  let db = client.database("nestbox");

  let nestboxes_col = db.collection("nestboxes");

  HttpServer::new(move || {
    let service_container = ServiceContainer::new(NestboxService::new(nestboxes_col.clone()));

    App::new()
      .data(AppState { service_container })
      .service(controller::nestboxes_get)
      
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
