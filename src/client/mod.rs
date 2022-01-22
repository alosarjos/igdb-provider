use crate::client::auth::APIAuth;
use crate::error::IGDBResult;
use crate::models::QueryableByName;
use reqwest::header::HeaderMap;
use reqwest::Response;

pub mod auth;

const IGDB_API_BASE_URL: &str = "https://api.igdb.com/v4";

pub struct Client {
    auth: APIAuth,
}

impl Client {
    pub fn new(auth: APIAuth) -> Self {
        Self { auth }
    }

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
}
