use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{bson::{extjson::de::Error}, sync::{Client}};
use mongodb::results::InsertOneResult;
use mongodb::sync::Database;
use serde::{Deserialize, Serialize};

pub struct MongoRepo {
    db: Database,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("seriousgame");
/*        let col: Collection<User> = db.collection("users");*/
        MongoRepo { db }
    }

    // create
    pub fn create<T:Serialize>(&self, collection_name: &str, new_item: T) ->Result<InsertOneResult, Error>{
        let item = self.db
            .collection(collection_name)
            .insert_one(new_item, None)
            .ok()
            .expect("Error creating");
        Ok(item)
    }

    // list
    pub fn list<T: Unpin + Sync + Send + for<'de> Deserialize<'de>>(&self, collection_name: &str) -> Result<Vec<T>, Error> {
        let cursors = self.db.collection(collection_name)
            .find(None, None)
            .ok()
            .expect("Error getting list");
        let items = cursors.map(|doc| doc.unwrap()).collect();
        Ok(items)
    }
}