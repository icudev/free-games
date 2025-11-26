pub mod twitter;

use utils::model::{Game, GameStore};

#[async_trait::async_trait]
pub trait Platform {
    fn name(&self) -> String;

    async fn post_game(&self, game: &Game) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn make_text(game: &Game) -> String {
    let mut hashtags = vec![];
    hashtags.push("#FreeGames".to_string());
    hashtags.push(format!("#{}", game.identifier));

    match game.store {
        GameStore::Steam => {
            hashtags.push("#SteamDeals".to_string());
        },
        GameStore::EpicGames => {},
        GameStore::Gog => {},
        GameStore::Unknown => {},
    }

    format!(
        "[ {} ] \"{}\" is currently free on #{:?} until {}.\n\n{}\n\n{}",
        game.game_type,
        game.title,
        game.store,
        game.offer_until.format("%Y-%m-%d"),
        game.url,
        hashtags.join(" "),
    )
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use super::*;
    use utils::model::GameType;

    #[test]
    fn test_game() {
        assert_eq!(
            make_text(&Game {
                id: "unique_id".to_string(),
                store: GameStore::Steam,
                title: "The Game".to_string(),
                identifier: "The_Game".to_string(),
                url: "https://icudev.xyz/the_game".to_string(),
                original_price: "$19.99".to_string(),
                offer_until: NaiveDate::from_ymd_opt(2025, 6, 15).unwrap(),
                game_type: GameType::Game,
            }),
            r#"[ Game ] "The Game" is currently free on #Steam until 2025-06-15.

https://icudev.xyz/the_game

#FreeGames #The_Game #SteamDeals"#
        );
    }

    #[test]
    fn test_dlc() {
        assert_eq!(
            make_text(&Game {
                id: "unique_dlc_id".to_string(),
                store: GameStore::Steam,
                title: "The Game: The DLC".to_string(),
                identifier: "The_Game_The_DLC".to_string(),
                url: "https://icudev.xyz/the_game_the_dlc".to_string(),
                original_price: "$9.99".to_string(),
                offer_until: NaiveDate::from_ymd_opt(2025, 6, 15).unwrap(),
                game_type: GameType::Dlc,
            }),
            r#"[ DLC ] "The Game: The DLC" is currently free on #Steam until 2025-06-15.

https://icudev.xyz/the_game_the_dlc

#FreeGames #The_Game_The_DLC #SteamDeals"#
        );
    }
}
