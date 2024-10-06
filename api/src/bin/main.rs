#[macro_use]
extern crate rocket;
use api::event_handler;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![
            event_handler::list_events_handler,
            event_handler::list_event_handler,
            event_handler::create_event_handler,
            event_handler::publish_event_handler,
            event_handler::delete_event_handler,
        ],
    )
}
