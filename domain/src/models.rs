pub mod data {
    use native_db::{native_db, ToKey};
    use native_model::{native_model, Model};
    use serde::{Deserialize, Serialize};
    use chrono::{serde, DateTime, Local};
    use uuid::Uuid;

    pub type Event = v1::Event;

    pub mod v1 {
        use super::*;

        #[derive(Serialize, Deserialize, Debug)]
        #[native_model(id = 1, version = 1)]
        #[native_db]
        pub struct Event {
            #[primary_key]
            pub id: Uuid,
            pub name: String,
            pub description: String,
            pub location: Option<String>,
            pub published: DateTime<Local>,
            pub updated: DateTime<Local>,
        }
    }
}
