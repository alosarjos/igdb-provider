use crate::error::IGDBResult;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

const TWITCH_OAUTH_REQUEST_URL: &str = "https://id.twitch.tv/oauth2/token";

pub struct APIAuth {
    pub client_id: String,
    client_secret: String,
    refresh_timestamp: Option<DateTime<Utc>>,
    pub token: Option<OAuthTokenData>,
}

impl APIAuth {
    pub fn new<T, S>(client_id: T, client_secret: S) -> Self
    where
        T: Into<String>,
        S: Into<String>,
    {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            refresh_timestamp: None,
            token: None,
        }
    }

    pub fn new_from_env() -> IGDBResult<Self> {
        let client_id = env::var("TWITCH_CLIENT_ID")?;
        let client_secret = env::var("TWITCH_CLIENT_SECRET")?;
        Ok(APIAuth::new(client_id, client_secret))
    }

    pub async fn request_token(&mut self) -> IGDBResult<()> {
        let client = reqwest::Client::new();
        let request_body = HashMap::from([
            ("client_id", self.client_id.as_ref()),
            ("client_secret", self.client_secret.as_ref()),
            ("grant_type", "client_credentials"),
        ]);

        let response = client
            .post(TWITCH_OAUTH_REQUEST_URL)
            .json(&request_body)
            .send()
            .await?;

        self.token = Some(response.json().await?);
        self.refresh_timestamp = Some(Utc::now());

        Ok(())
    }

    pub fn is_token_valid(&self) -> bool {
        match (self.refresh_timestamp.as_ref(), self.token.as_ref()) {
            (Some(refresh_timestamp), Some(token)) => {
                refresh_timestamp.timestamp() + token.expires_in > Utc::now().timestamp()
            }
            _ => false,
        }
    }

    pub async fn refresh_token(&mut self) -> IGDBResult<()> {
        // TODO: Refresh the token properly instead of regenerate it
        self.request_token().await
    }
}

#[derive(Deserialize)]
pub struct OAuthTokenData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[cfg(test)]
mod tests {
    use crate::client::auth::APIAuth;

    #[tokio::test]
    async fn request_twitch_oauth_token() {
        let mut auth = APIAuth::new_from_env().unwrap();
        assert!(!auth.is_token_valid());

        auth.request_token().await.unwrap();
        assert!(auth.is_token_valid());
    }
}
