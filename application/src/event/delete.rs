use diesel::prelude::*;
use domain::models::Event;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn delete_event(event_id: i32) -> Result<Vec<Event>, NotFound<String>> {
    use domain::schema::events;
    use domain::schema::events::dsl::*;

    let response: Response;

    let num_deleted =
        match diesel::delete(events.filter(id.eq(event_id))).execute(&mut establish_connection()) {
            Ok(count) => count,
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    let response = Response {
                        body: ResponseBody::Message(format!(
                            "Error deleting event with id {} - {}",
                            event_id, err
                        )),
                    };
                    return Err(NotFound(serde_json::to_string(&response).unwrap()));
                }
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        };

    if num_deleted > 0 {
        match events::table
            .select(events::all_columns)
            .load::<Event>(&mut establish_connection())
        {
            Ok(mut events_) => {
                events_.sort();
                Ok(events_)
            }
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        }
    } else {
        response = Response {
            body: ResponseBody::Message(format!("Error - no event with id {}", event_id)),
        };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}
