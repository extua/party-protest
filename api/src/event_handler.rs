use shared::response_models::{Response, ResponseBody};
use application::event::{read};
use domain::models::{Event};
use rocket::{get};
use rocket::response::status::{NotFound};
use rocket::serde::json::Json;

#[get("/")]
pub fn list_events_handler() -> String {
    let posts: Vec<Event> = read::list_events();
    let response = Response { body: ResponseBody::Events(events) };

    serde_json::to_string(&response).unwrap()
}

#[get("/event/<event_id>")]
pub fn list_event_handler(event_id: i32) -> Result<String, NotFound<String>> {
    let event = read::list_event(event_id)?;
    let response = Response { body: ResponseBody::Event(event) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[event("/new_event", format = "application/json", data = "<event>")]
pub fn create_event_handler(post: Json<NewEvent>) -> Created<String> {
    create::create_event(event)
}