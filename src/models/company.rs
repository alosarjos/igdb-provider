use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Company {
    pub id: i32,
    pub name: String,
}
