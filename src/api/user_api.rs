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
    let new_user = User {
        id: None,
        firstName: new_user.firstName.to_owned(),
        lastName: new_user.lastName.to_owned(),
        email: new_user.email.to_owned(),
        role: new_user.role.to_owned(),
    };
    let user_detail = db.create::<User>("users",new_user);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}