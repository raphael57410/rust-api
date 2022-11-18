use argon2::{Config};
use mongodb::results::InsertOneResult;
use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};


#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.list::<User>("users");
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[post("/users", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {

    // hashage du mot de passe
    let binding = new_user.password.to_owned();
    let salt = b"randomsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(binding.as_ref(), salt, &config).unwrap();

    let new_user = User {
        id: None,
        firstName: new_user.firstName.to_owned(),
        lastName: new_user.lastName.to_owned(),
        password: hash.to_string(),
        email: new_user.email.to_owned(),
        role: new_user.role.to_owned(),
    };
    let user_detail = db.create::<User>("users",new_user);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users/<user_id>")]
pub fn get_one_user(db: &State<MongoRepo>, user_id: String) -> Result<Json<User>, Status> {
    let id = user_id;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_one("users", &id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/users/<user_id>")]
pub fn delete_user(db: &State<MongoRepo>, user_id: String) -> Result<Json<User>, Status> {
    let id = user_id;

    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let user_detail = db.delete::<User>("users", &id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}