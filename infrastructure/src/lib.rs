use native_db::*;
use once_cell::sync::Lazy;
use domain::models;

static MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<data::v1::Event>().unwrap();
    models
 });

fn main() -> Result<(), db_type::Error> {
    // Create the database
    let db = Builder::new().create_in_memory(&MODELS)?;
    Ok(())
}