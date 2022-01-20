use crate::models::{IGDBQueryable, IGDBQueryableByName};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Game {
    pub name: String,
}

impl IGDBQueryableByName for Game {}

impl IGDBQueryable for Game {
    fn get_endpoint() -> &'static str {
        "games"
    }

    fn get_fields() -> &'static str {
        "id,
	    artworks.id, artworks.url,
	    category,
	    collection,
	    cover.id, cover.url,
	    dlcs,
	    expansions,
	    first_release_date,
	    game_modes,
	    genres,
	    involved_companies.developer, involved_companies.publisher, involved_companies.company.name,
	    name,
	    platforms,
	    screenshots.url,
	    similar_games,
	    standalone_expansions,
	    videos.video_id"
    }

    fn get_filters() -> &'static str {
        "cover != null & version_parent = null"
    }
}
