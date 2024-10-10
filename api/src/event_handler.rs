use application::event::{create, delete, publish, read};
use domain::models::data::v1::Event;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::response_models::{Response, ResponseBody};

#[get("/")]
pub fn list_events_handler() -> String {
    let events: Vec<Event> = read::list_events();
    let response = Response {
        body: ResponseBody::Events(events),
    };

    serde_json::to_string(&response).unwrap()
}

#[get("/event/<event_id>")]
pub fn list_event_handler(event_id: i32) -> Result<String, NotFound<String>> {
    let event = read::list_event(event_id)?;
    let response = Response {
        body: ResponseBody::Event(event),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_event", format = "application/json", data = "<event>")]
pub fn create_event_handler(event: Json<NewEvent>) -> Created<String> {
    create::create_event(event)
}

#[get("/publish/<event_id>")]
pub fn publish_event_handler(event_id: i32) -> Result<String, NotFound<String>> {
    let event = publish::publish_event(event_id)?;
    let response = Response {
        body: ResponseBody::Event(event),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/delete/<event_id>")]
pub fn delete_event_handler(event_id: i32) -> Result<String, NotFound<String>> {
    let events = delete::delete_event(event_id)?;
    let response = Response {
        body: ResponseBody::Events(events),
    };

    Ok(serde_json::to_string(&response).unwrap())
}
