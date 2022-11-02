#[macro_use] extern crate rocket;
mod api;
mod models;
mod repository;
mod crud_ctrl;

use crud_ctrl::get;

// import de la class mongoDb
use crate::repository::mongodb_repo::MongoRepo;

// import des routes de user
use api::user_api::create_user;
use api::user_api::get_all_users;

#[get("/world")]
fn world() {
    get((), Default::default())
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/api", routes![world,create_user,get_all_users])
}