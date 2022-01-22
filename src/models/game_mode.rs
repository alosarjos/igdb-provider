use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GameMode {
    pub id: i32,
    pub name: String,
}
