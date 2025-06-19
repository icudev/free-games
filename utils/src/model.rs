use std::fmt;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Game {
    pub id: String,
    pub store: GameStore,
    pub title: String,
    pub identifier: String,
    pub url: String,
    pub original_price: String,
    pub offer_until: NaiveDate,
    pub game_type: GameType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialGame {
    pub id: String,
    pub store: GameStore,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameStore {
    #[serde(rename = "Steam")]
    Steam,
    #[serde(rename = "EpicGames")]
    EpicGames,
    #[serde(rename = "GOG")]
    Gog,
    #[serde(other)]
    Unknown,
}

impl fmt::Display for GameStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameStore::Steam => write!(f, "Steam"),
            GameStore::EpicGames => write!(f, "EpicGames"),
            GameStore::Gog => write!(f, "GOG"),
            GameStore::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<String> for GameStore {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "steam" => GameStore::Steam,
            "epicgames" => GameStore::EpicGames,
            "gog" => GameStore::Gog,
            _ => GameStore::Unknown,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameType {
    Game,
    Dlc,
    Software,
    Bundle,
    Edition,
    Unknown,
}

impl fmt::Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameType::Game => write!(f, "Game"),
            GameType::Dlc => write!(f, "DLC"),
            GameType::Software => write!(f, "Software"),
            GameType::Bundle => write!(f, "Bundle"),
            GameType::Edition => write!(f, "Edition"),
            GameType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<String> for GameType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "game" => GameType::Game,
            "dlc" => GameType::Dlc,
            "software" => GameType::Software,
            "bundle" => GameType::Bundle,
            "edition" => GameType::Edition,
            _ => GameType::Unknown,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostedPlatform {
    pub platform: String,
    pub game_id: String,
    pub game_store: GameStore,
}
