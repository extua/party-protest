use crate::schema::events;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub location: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub name: String,
    pub description: String,
    pub location: Option<String>
}
