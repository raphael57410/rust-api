use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::{models::auth::Auth, repository::mongodb_repo::MongoRepo};
use crate::models::user_model::User;

#[post("/auth", data = "<form_login>")]
pub fn auth(
    db: &State<MongoRepo>,
    form_login: Json<Auth>,
) -> Status {

    // Récuperer un user via son email si il existe
    let email = form_login.email.to_owned();
    let user  = db.get_one_by_email::<User>(&email);

    // Vérifier si le password est correct
        let user_encoded_password = &user.as_ref().unwrap().password;
        let form_password = &form_login.password;
        let res = argon2::verify_encoded(&user_encoded_password, (&form_password).as_ref()).unwrap();

    // renvoi au front une réponse si la vérification c'est bien passé ou pas
    match res {
        true => Status::Ok,
        false => Status::NotFound,
    }
}