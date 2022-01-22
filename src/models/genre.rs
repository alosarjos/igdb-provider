use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Genre {
    pub id: i32,
    pub name: String,
}
