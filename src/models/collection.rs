use crate::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GameCollection {
    pub id: i32,
    pub name: String,
    pub games: Vec<i32>,
}

impl Queryable for GameCollection {
    fn get_endpoint() -> &'static str {
        "collections"
    }

    fn get_fields() -> &'static str {
        "id, name, games"
    }

    fn get_filters() -> &'static str {
        ""
    }
}
