use reqwest::Client;
use utils::internal_api::{wait_for_internal_api, InternalApi};
use crate::stores::{EpicGamesStore, SteamStore, Store};

mod stores;



const INTERVAL: u64 = 600;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::logging::setup_logger().expect("TODO: panic message");

    let http = Client::new();
    let api_url = std::env::var("INTERNAL_API_URL").unwrap();
    let api_token = std::env::var("INTERNAL_API_AUTH_TOKEN").unwrap();
    let internal_api = InternalApi::new(api_url, api_token);

    if let Err(e) = wait_for_internal_api(&internal_api).await {
        return Err(format!("Error while connecting to internal API: {e}").into());
    }

    let stores: Vec<Box<dyn Store>> = vec![
        Box::new(EpicGamesStore),
        Box::new(SteamStore),
    ];

    main_loop(&http, &stores, &internal_api).await;

    Ok(())
}

async fn main_loop(http: &Client, stores: &Vec<Box<dyn Store>>, internal_api: &InternalApi) {
    loop {
        log::debug!("Searching for games...");

        for store in stores.iter() {
            let games = store.get_games(http, internal_api).await;

            for game in games {
                log::info!("Posting Game {:?} to API", game.title);

                let _ = internal_api.post_game(&game).await;

                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(INTERVAL)).await;
    }
}
