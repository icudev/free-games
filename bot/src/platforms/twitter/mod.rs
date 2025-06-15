use std::error::Error;
use std::env;
use twitter_v2::authorization::Oauth1aToken;
use twitter_v2::TwitterApi;
use utils::model::Game;
use crate::platforms::{make_text, Platform};

pub struct TwitterPlatform {
    client: TwitterApi<Oauth1aToken>,
}

impl TwitterPlatform {
    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: TwitterApi::new(Oauth1aToken::new(
                env::var("TWITTER_API_KEY")?,
                env::var("TWITTER_API_KEY_SECRET")?,
                env::var("TWITTER_ACCESS_TOKEN")?,
                env::var("TWITTER_ACCESS_TOKEN_SECRET")?,
            ))
        })
    }
}

#[async_trait::async_trait]
impl Platform for TwitterPlatform {
    fn name(&self) -> String {
        String::from("Twitter")
    }

    async fn post_game(&self, game: &Game) -> Result<(), Box<dyn Error>> {
        let res = self.client
            .post_tweet()
            .text(make_text(game))
            .send()
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(error) => Err(error.into()),
        }
    }
}
