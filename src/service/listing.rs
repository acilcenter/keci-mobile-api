use bson::{doc, ordered};
use mongodb::options::FindOptions;
use mongodb::Collection;
use std::vec;

pub fn get(
  collection: Collection,
) -> Result<std::vec::Vec<bson::ordered::OrderedDocument>, String> {
  match collection.find(
    doc! {"visible": true},
    FindOptions::builder().sort(doc! { "priority": -1 }).build(),
  ) {
    Ok(cursor) => {
      let mut listings: Vec<ordered::OrderedDocument> = vec![];
      for result in cursor {
        if let Ok(document) = result {
          listings.push(document);
        } else {
          return Err(String::from("Can't find listings"));
        }
      }
      Ok(listings)
    }
    Err(_e) => Err(String::from("Error while getting listings")),
  }
}
