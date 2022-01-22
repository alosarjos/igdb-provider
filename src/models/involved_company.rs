use crate::models::Company;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvolvedCompany {
    pub id: i32,
    pub developer: bool,
    pub publisher: bool,
    pub company: Company,
}
