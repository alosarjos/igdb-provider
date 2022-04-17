use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Cover {
    pub id: i32,
    #[serde(deserialize_with = "cover_url_mapper")]
    pub url: String,
}

pub fn cover_url_mapper<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    Ok(format!(
        "https:{}",
        value.replace("t_thumb", "t_cover_big_2x"),
    ))
}
