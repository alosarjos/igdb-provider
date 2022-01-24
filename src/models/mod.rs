mod collection;
mod company;
mod game;
mod game_mode;
mod genre;
mod involved_company;
mod platform;
mod screenshot;
mod video;

use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub use collection::*;
pub use company::*;
pub use game::*;
pub use game_mode::*;
pub use genre::*;
pub use involved_company::*;
pub use platform::*;
pub use screenshot::*;
pub use video::*;

pub trait Queryable: Serialize + DeserializeOwned + Debug {
    fn get_endpoint() -> &'static str;
    fn get_fields() -> &'static str;
    fn get_filters() -> &'static str;
}

pub trait QueryableByName: Queryable {}
