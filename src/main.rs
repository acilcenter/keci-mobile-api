use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client, Collection};

mod controller;
mod middleware;
mod service;

pub struct AppState {
  listing_collection: Collection,
  content_collection: Collection,
  user_collection: Collection,
  basket_collection: Collection,
}

#[macro_use]
extern crate dotenv_codegen;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let client_options = ClientOptions::parse(dotenv!("DB_URL")).unwrap();
  let client = Client::with_options(client_options).unwrap();
  let db = client.database(dotenv!("DB_NAME"));
  let listing_collection = db.collection(dotenv!("DB_LISTING_COLLECTION"));
  let content_collection = db.collection(dotenv!("DB_CONTENT_COLLECTION"));
  let user_collection = db.collection(dotenv!("DB_USER_COLLECTION"));
  let basket_collection = db.collection(dotenv!("DB_BASKET_COLLECTION"));

  HttpServer::new(move || {
    App::new()
      .data(AppState {
        listing_collection: listing_collection.clone(),
        content_collection: content_collection.clone(),
        user_collection: user_collection.clone(),
        basket_collection: basket_collection.clone(),
      })
      .service(
        web::scope("/listings")
          .wrap(middleware::user::Resolve)
          .route("", web::get().to(controller::listing::get)),
      )
      .service(
        web::scope("/contents")
          .wrap(middleware::user::Resolve)
          .route("", web::get().to(controller::content::get)),
      )
      .service(
        web::scope("/basket")
          .wrap(middleware::user::Resolve)
          .route("", web::post().to(controller::basket::add)),
      )
      .service(web::scope("/users").route("", web::post().to(controller::user::create)))
  })
  .bind("0.0.0.0:3003")?
  .run()
  .await
}
