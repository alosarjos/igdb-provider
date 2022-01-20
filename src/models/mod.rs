use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod game;

pub trait IGDBQueryable: Serialize + DeserializeOwned {
    fn get_endpoint() -> &'static str;
    fn get_fields() -> &'static str;
    fn get_filters() -> &'static str;
}

pub trait IGDBQueryableByName: IGDBQueryable {}
