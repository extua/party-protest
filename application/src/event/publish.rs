use domain::models::Event;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use diesel::prelude::*;

pub fn publish_event(event_id: i32) -> Result<Event, NotFound<String>> {
    use domain::schema::events::dsl::*;

    match diesel::update(events.find(event_id)).set(published.eq(true)).get_result::<Event>(&mut establish_connection()) {
        Ok(event) => Ok(event),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing event with id {} - {}", event_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}