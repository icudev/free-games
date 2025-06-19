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
