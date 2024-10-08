pub mod data {
    use chrono::{DateTime, Local};
    use native_db::{native_db, ToKey};
    use native_model::{native_model, Model};
    use serde::{Deserialize, Serialize};

    pub type Event = v1::Event;

    pub mod v1 {
        use super::*;

        #[derive(Serialize, Deserialize, Debug)]
        #[native_model(id = 1, version = 1)]
        #[native_db]
        pub struct Event {
            #[primary_key]
            pub id: String,
            pub name: String,
            pub description: String,
            pub location: Option<String>,
            pub published: DateTime<Local>,
            pub updated: DateTime<Local>,
        }
    }
}
