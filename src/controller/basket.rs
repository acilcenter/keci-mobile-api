use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Body {
  pub product_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
  pub id: bson::Bson,
  pub message: String,
}

pub async fn add(
  request: HttpRequest,
  app_data: web::Data<crate::AppState>,
  body: web::Json<Body>,
) -> impl Responder {
  let product_id = body.product_id.clone();
  match request.headers().get("user_id") {
    Some(user_id_header) => {
      // user
      match user_id_header.to_str() {
        Ok(user_id_str) => {
          let user_id = String::from(user_id_str);
          web::block(move || {
            crate::service::basket::get_active(app_data.basket_collection.clone(), user_id)
          })
          .await
          .map(|(active_basket_option, collection, user_id)| {
            match active_basket_option {
              Some(active_basket) => {
                // user has active basket

                // check whether product is already in active basket
                println!("active_basket: {:?}", active_basket);

                HttpResponse::Ok().json("user has active basket")
              }
              None => {
                // user does not have active basket
                // TODO: wrap with web::block
                let basket_result = crate::service::basket::create(collection, &product_id, &user_id);
                match basket_result {
                  Ok(basket) => {
                    let response = Response {
                      id: basket.inserted_id,
                      message: String::from("created active basket fro user")
                    };
                    HttpResponse::Ok().json(response)
                  },
                  Err(_e) => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
                }
              }
            }
          })
        }
        Err(e) => {
          println!(
            "Error while getting string of user_id header value, {:?}",
            e
          );
          Ok(HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR))
        }
      }
    }
    None => {
      // anon
      Ok(HttpResponse::Ok().json("anon"))
    }
  }
}
