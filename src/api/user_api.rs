use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        firstName: new_user.firstName.to_owned(),
        lastName: new_user.lastName.to_owned(),
        email: new_user.email.to_owned(),
        role: new_user.role.to_owned(),
    };
    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}