#[macro_use] extern crate rocket;
mod api;
mod models;
mod repository;

// import de la class mongoDb
use crate::repository::mongodb_repo::MongoRepo;

// import des routes de user
use api::user_api::{get_all_users,create_user};


#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/api", routes![get_all_users,create_user])
}