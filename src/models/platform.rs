use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Platform {
    pub id: i32,
    pub name: String,
}
