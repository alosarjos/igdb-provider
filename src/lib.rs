mod client;
pub mod error;
mod models;

pub use client::{auth::APIAuth, Client};
pub use models::*;
