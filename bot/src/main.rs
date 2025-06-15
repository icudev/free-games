use utils::internal_api::{wait_for_internal_api, InternalApi};
use utils::model::{Game, PostedPlatform};
use crate::platforms::Platform;
use crate::platforms::twitter::TwitterPlatform;

mod platforms;

type Error = Box<dyn std::error::Error>;

const INTERVAL: u64 = 600;

#[tokio::main]
async fn main() -> Result<(), Error> {
    utils::logging::setup_logger()?;

    let platforms: Vec<Box<dyn Platform>> = vec![
        Box::new(TwitterPlatform::new()?)
    ];

    let api_url = std::env::var("INTERNAL_API_URL")?;
    let api_token = std::env::var("INTERNAL_API_AUTH_TOKEN")?;
    let internal_api = InternalApi::new(api_url, api_token);

    main_loop(&internal_api, &platforms).await?;

    Ok(())
}

async fn main_loop(internal_api: &InternalApi, platforms: &Vec<Box<dyn Platform>>) -> Result<(), Error> {
    if let Err(e) = wait_for_internal_api(internal_api).await {
        return Err(format!("Error while connecting to internal API: {e}").into());
    }

    loop {
        let games = internal_api.get_free_games().await?;

        log::debug!("Found {} free games, dispatching.", games.len());

        dispatch_games(internal_api, platforms, &games).await?;

        tokio::time::sleep(std::time::Duration::from_secs(INTERVAL)).await;
    }
}

async fn dispatch_games(internal_api: &InternalApi, platforms: &Vec<Box<dyn Platform>>, games: &Vec<Game>) -> Result<(), Error> {
    for game in games {
        let game_id = game.id.clone();
        let game_store = game.store.clone();

        for platform in platforms {
            let posted = PostedPlatform {
                game_id: game_id.clone(),
                game_store: game_store.clone(),
                platform: platform.name()
            };

            if internal_api.is_posted(&posted).await? {
                log::debug!("Already posted game {game_id}, skipping");
                continue;
            }

            log::debug!("Posting game {game_id}");
            match platform.post_game(game).await {
                Ok(_) => {
                    let _ = internal_api.post_posted(&posted).await?;
                    log::info!("Posted game \"{}\" to platform: {}", game.id, platform.name());
                },
                Err(e) => {
                    log::error!("Failed to post game \"{}\" to platform: {}", game.id, e);
                },
            };
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
