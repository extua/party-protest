#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;


mod tera;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![world])
        .mount("/tera", routes![tera::index, tera::hello, tera::about])
        .register("/tera", catchers![tera::not_found])
        .attach(Template::custom(|engines| {
            tera::customize(&mut engines.tera);
        }))
    .mount("/assets", FileServer::from(relative!("/assets")))
}

