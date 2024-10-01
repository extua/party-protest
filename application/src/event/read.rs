use diesel::prelude::*;
use domain::models::Event;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn list_event(event_id: i32) -> Result<Event, NotFound<String>> {
    use domain::schema::events;

    match events::table
        .find(event_id)
        .first::<Event>(&mut establish_connection())
    {
        Ok(event) => Ok(event),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error selecting event with id {} - {}",
                        event_id, err
                    )),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_events() -> Vec<Event> {
    use domain::schema::events;

    match events::table
        .select(events::all_columns)
        .load::<Event>(&mut establish_connection())
    {
        Ok(mut events) => {
            events.sort();
            events
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
