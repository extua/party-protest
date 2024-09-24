use crate::schema::events;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub start_time: u64,
    pub end_time: u64,
    pub location: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub name: String,
    pub description: String,
}