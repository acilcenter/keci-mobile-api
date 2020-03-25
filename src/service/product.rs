use bson::{doc, ordered};
use mongodb::Collection;
use std::vec;

pub fn get(
  collection: Collection,
) -> Result<std::vec::Vec<bson::ordered::OrderedDocument>, String> {
  match collection.find(doc! {}, None) {
    Ok(cursor) => {
      let mut products: Vec<ordered::OrderedDocument> = vec![];
      for result in cursor {
        if let Ok(document) = result {
          products.push(document);
        } else {
          return Err(String::from("Can't find products"));
        }
      }
      Ok(products)
    }
    Err(_e) => Err(String::from("Error while getting products")),
  }
}
