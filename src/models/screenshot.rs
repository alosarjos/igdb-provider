use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Screenshot {
    pub id: i32,
    #[serde(deserialize_with = "screenshot_url_mapper")]
    pub url: String,
}

pub fn screenshot_url_mapper<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    Ok(format!(
        "https:{}",
        value.replace("t_thumb", "t_screenshot_huge"),
    ))
}
