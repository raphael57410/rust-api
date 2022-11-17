use rocket::http::{CookieJar,Cookie};

#[get("/")]
pub fn home(cookies: &CookieJar<'_>) {
    let cookie = Cookie::new("name", "value");
    cookies.add_private(cookie);
}