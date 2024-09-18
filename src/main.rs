#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![world])
    .mount("/assets", FileServer::from(relative!("/assets")))
}

