use serde::{Deserialize, Deserializer};
use utils::model::GameType;

#[derive(Debug, Deserialize)]
pub(crate) struct ApiResponse {
    pub products: Vec<GogGame>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GogGame {
    pub id: String,
    #[serde(deserialize_with = "deserialize_game_type")]
    pub product_type: GameType,
    pub title: String,
    pub price: Price,
    pub store_link: String,
}

fn deserialize_game_type<'de, D>(deserializer: D) -> Result<GameType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "game" => Ok(GameType::Game),
        "dlc" => Ok(GameType::Dlc),
        "pack" => Ok(GameType::Edition),
        _ => Ok(GameType::Unknown),
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Price {
    pub r#final: String,
    pub base: String,
    pub discount: String,
}
