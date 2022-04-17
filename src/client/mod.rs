use crate::client::auth::APIAuth;
use crate::error::IGDBResult;
use crate::models::{Queryable, QueryableByName};
use reqwest::header::HeaderMap;
use reqwest::Response;

pub mod auth;

const IGDB_API_BASE_URL: &str = "https://api.igdb.com/v4";

/**
 * The client provides the functionality required to make requests to the IGDB
 * API and parses the results into the models provided by the crate
 */
pub struct Client {
    auth: APIAuth,
}

impl Client {
    pub fn new(auth: APIAuth) -> Self {
        Self { auth }
    }

    /**
     * Queries for a single item by Id and returns a deserialized response
     */
    pub async fn query<T, R>(&self, id: T) -> IGDBResult<Option<R>>
    where
        T: Into<String>,
        R: Queryable,
    {
        let id = id.into();
        let fields = R::get_fields();

        let request_body = format!("fields {fields}; where id = {id};");
        let response = self.request(R::get_endpoint(), request_body).await?;

        let results: Vec<R> = response.json().await?;
        Ok(results.into_iter().next())
    }

    pub fn is_token_valid(&self) -> bool {
        self.auth.is_token_valid()
    }

    pub async fn refresh_auth_token(&mut self) -> IGDBResult<()> {
        Ok(self.auth.refresh_token().await?)
    }

    /**
     * Queries for multiple items by name and returns a deserialized response
     */
    pub async fn query_by_name<T, R>(&self, query: T) -> IGDBResult<Vec<R>>
    where
        T: Into<String>,
        R: QueryableByName,
    {
        let query = query.into();
        let fields = R::get_fields();
        let filters = R::get_filters();

        let request_body = format!("search \"{query}\"; fields {fields}; where {filters};");
        let response = self.request(R::get_endpoint(), request_body).await?;

        Ok(response.json().await?)
    }

    async fn request<T>(&self, endpoint: &str, body: T) -> IGDBResult<Response>
    where
        T: Into<String>,
    {
        let client = reqwest::Client::new();

        let url = format!("{IGDB_API_BASE_URL}/{endpoint}");
        let request_headers = self.get_request_headers().await?;

        let response = client
            .post(url)
            .headers(request_headers)
            .body(body.into())
            .send()
            .await?;

        Ok(response)
    }

    async fn get_request_headers(&self) -> IGDBResult<HeaderMap> {
        let token_data = self.auth.token.as_ref().unwrap();
        let token = &token_data.access_token;

        let mut headers = HeaderMap::new();
        headers.insert("Client-ID", self.auth.client_id.parse().unwrap());
        headers.insert("Authorization", format!("Bearer {token}").parse().unwrap());

        Ok(headers)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::auth::APIAuth;
    use crate::client::Client;
    use crate::models::Game;
    use crate::models::GameCollection;

    #[tokio::test]
    async fn query_games_by_name() {
        let mut auth = APIAuth::new_from_env().unwrap();
        auth.request_token().await.unwrap();
        assert!(auth.is_token_valid());

        let client = Client::new(auth);
        let games: Vec<Game> = client.query_by_name("The Witcher 3").await.unwrap();

        let game = games
            .iter()
            .find(|game| game.name == "The Witcher 3: Wild Hunt");
        assert!(game.is_some());
    }

    #[tokio::test]
    async fn query_game_by_id() {
        let mut auth = APIAuth::new_from_env().unwrap();
        auth.request_token().await.unwrap();
        assert!(auth.is_token_valid());

        let client = Client::new(auth);
        let game: Option<Game> = client.query("1942").await.unwrap();
        let game = game.unwrap();
        assert_eq!(game.name, "The Witcher 3: Wild Hunt");
    }

    #[tokio::test]
    async fn query_game_collection_by_id() {
        let mut auth = APIAuth::new_from_env().unwrap();
        auth.request_token().await.unwrap();
        assert!(auth.is_token_valid());

        let client = Client::new(auth);
        let game_collection: Option<GameCollection> = client.query("62").await.unwrap();
        let game_collection = game_collection.unwrap();
        assert_eq!(game_collection.name, "The Witcher");
        assert_eq!(game_collection.games.len(), 20);
    }
}
