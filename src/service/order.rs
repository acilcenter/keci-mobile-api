use crate::model::order::Order;
use crate::traits::service::{Creator, Getter};
use bson::{doc, oid::ObjectId, ordered, to_bson, Bson};
use mongodb::error::{Error, ErrorKind};
use mongodb::results::InsertOneResult;
use mongodb::Collection;

#[derive(Clone)]
pub struct OrderService {
  collection: Collection,
}

impl OrderService {
  pub fn new(collection: Collection) -> Self {
    OrderService { collection }
  }

  pub fn find(&self, id: &str, user_id: &str) -> Result<Option<bson::ordered::OrderedDocument>, Error> {
    self.collection.find_one(
      doc! {"_id": ObjectId::with_string(id).expect("Id not valid"), "user_id": ObjectId::with_string(user_id).expect("user_id not valid")},
      None,
    )
  }
}

impl Creator<Order> for OrderService {
  fn create(&self, order: &Order) -> Result<InsertOneResult, Error> {
    let serialized_order = to_bson(&order).unwrap();
    if let Bson::Document(mut document) = serialized_order {
      document.insert("created_at", chrono::Utc::now());
      match self.collection.insert_one(document, None) {
        Ok(insert_result) => Ok(insert_result),
        Err(e) => Err(e),
      }
    } else {
      Err(Error::from(ErrorKind::OperationError {
        message: String::from("Can not create order"),
      }))
    }
  }
}

impl Getter for OrderService {
  fn get_all(&self, id: &str) -> Result<std::vec::Vec<bson::ordered::OrderedDocument>, String> {
    match self.collection.find(
      doc! {"user_id": ObjectId::with_string(id).expect("user_id is not valid")},
      None,
    ) {
      Ok(cursor) => {
        let mut orders: Vec<ordered::OrderedDocument> = vec![];
        for result in cursor {
          if let Ok(document) = result {
            orders.push(document);
          } else {
            return Err(String::from("Can't find orders"));
          }
        }
        Ok(orders)
      }
      Err(_e) => Err(String::from("Error while getting orders")),
    }
  }
}
