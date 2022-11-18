#[macro_use] extern crate rocket;
mod api;
mod models;
mod repository;

// import de la class mongoDb
use crate::repository::mongodb_repo::MongoRepo;

// import des routes de user
use api::user_api::{get_all_users,create_user,get_one_user,delete_user};
use api::home_api::home;
use api::auth_api::auth;


#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/api",routes![auth])
        .mount("/api",routes![home])
        .mount("/api", routes![get_all_users,create_user,get_one_user,delete_user])
}