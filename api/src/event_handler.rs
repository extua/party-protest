use shared::response_models::{Response, ResponseBody};
use application::event::{read};
use domain::models::{Event};
use rocket::{get};
use rocket::response::status::{NotFound};
use rocket::serde::json::Json;

#[get("/")]
pub fn list_events_handler() -> String {
    todo!()
}

#[get("/event/<event_id>")]
pub fn list_event_handler(event_id: i32) -> Result<String, NotFound<String>> {
    todo!()
}