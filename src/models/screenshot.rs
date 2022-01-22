use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Screenshot {
    pub id: i32,
    pub url: String,
}
