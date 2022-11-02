use rocket::{Request, Response};
use rocket::serde::json::Json;

fn generic_ctrl_fn(_res:Response, identifier: &str, service_call_fn: fn()) {

    println!("Nom => {}",identifier);
    service_call_fn();

    /*let test = match service_call_fn() {
        Ok(response) => response,
        Err(err) => err,
    };*/
}


pub fn get(_req: (), res: Response) {
    generic_ctrl_fn(res,"test du get!",|| {println!("Another function.")});
}