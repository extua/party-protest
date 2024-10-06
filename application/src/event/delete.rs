use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use domain::models::Event;

pub fn delete_event(event_id: i32) -> Result<Vec<Event>, NotFound<String>> {
    use domain::schema::events::dsl::*;
    use domain::schema::events;

    let response: Response;

    let num_deleted = match diesel::delete(events.filter(id.eq(event_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error deleting event with id {} - {}", event_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match events::table.select(events::all_columns).load::<Event>(&mut establish_connection()) {
            Ok(mut events_) => {
                events_.sort();
                Ok(events_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no event with id {}", event_id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}