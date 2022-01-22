use crate::models::{
    GameCollection, GameMode, Genre, InvolvedCompany, Platform, Queryable, QueryableByName,
    Screenshot, Video,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub type GameId = i32;

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub id: GameId,
    pub name: String,
    pub category: Category,
    pub collection: Option<GameCollection>,
    pub cover: Cover,
    pub dlcs: Option<Vec<GameId>>,
    pub expansions: Option<Vec<GameId>>,
    pub first_release_date: Option<i32>,
    pub game_modes: Option<Vec<GameMode>>,
    pub genres: Option<Vec<Genre>>,
    pub involved_companies: Option<Vec<InvolvedCompany>>,
    pub platforms: Option<Vec<Platform>>,
    pub screenshots: Option<Vec<Screenshot>>,
    pub similar_games: Option<Vec<GameId>>,
    pub standalone_expansions: Option<Vec<GameId>>,
    pub videos: Option<Vec<Video>>,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Category {
    MainGame = 0,
    DlcAddon = 1,
    Expansion = 2,
    Bundle = 3,
    StandaloneExpansion = 4,
    Mod = 5,
    Episode = 6,
    Season = 7,
    Remake = 8,
    Remaster = 9,
    ExpandedGame = 10,
    Port = 11,
    Fork = 12,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cover {
    pub id: i32,
    pub url: String,
}

impl QueryableByName for Game {}

impl Queryable for Game {
    fn get_endpoint() -> &'static str {
        "games"
    }

    fn get_fields() -> &'static str {
        "id,
	    category,
	    collection.name, collection.games,
	    cover.url,
	    dlcs,
	    expansions,
	    first_release_date,
	    game_modes.name,
	    genres.name,
	    involved_companies.developer, involved_companies.publisher, involved_companies.company.name,
	    name,
	    platforms.name,
	    screenshots.url,
	    similar_games,
	    standalone_expansions,
	    videos.video_id, videos.name"
    }

    fn get_filters() -> &'static str {
        "cover != null & version_parent = null"
    }
}
