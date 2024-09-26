#[macro_use] extern crate rocket;
use api::event_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            event_handler::list_events_handler, 
            event_handler::list_event_handler,
        ])
}