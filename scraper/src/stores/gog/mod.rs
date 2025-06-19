mod model;

use std::str::FromStr;
use chrono::NaiveDate;
use regex::Regex;
use reqwest::{Client, Url};
use reqwest::header::{HeaderMap, COOKIE};
use scraper::{Html, Selector};
use utils::internal_api::InternalApi;
use utils::model::{Game, GameStore, GameType, PartialGame};
use crate::stores::{make_api_request, make_identifier, make_request, Store};

pub struct GogStore;

const GOG_API_URL: &str = "https://catalog.gog.com/v1/catalog?limit=48&price=between:0,0&order=desc:trending&discounted=eq:true&productType=in:game,pack,dlc,extras&page=1&countryCode=US&locale=en-US&currencyCode=USD";

const COOKIES: &str = "gog_wantsmaturecontent=18;";

#[async_trait::async_trait]
impl Store for GogStore {
    async fn get_games(&self, http: &Client, internal_api: &InternalApi) -> Vec<Game> {
        let url = Url::from_str(GOG_API_URL).unwrap();
        let api_response = match make_api_request::<model::ApiResponse>(http, &url, HeaderMap::new()).await {
            Ok(response) => response,
            Err(e) => {
                log::error!("Error getting API response: {e}");
                return vec![];
            }
        };
        
        let games = api_response.products;
        let mut free_games = Vec::with_capacity(games.len());
        
        for game in games {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            
            if game.price.discount != "-100%" || game.price.r#final != "$0.00" {
                continue;
            }

            let partial_game = PartialGame {
                id: game.id.clone(),
                store: GameStore::Gog,
            };

            match internal_api.get_game(&partial_game).await {
                Ok(false) => {
                    log::debug!("Game {} does not exist.", game.id)
                },
                _ => {
                    log::debug!("Game {} already exists, skipping.", game.id);
                    continue;
                }
            }
            
            let store_link = Url::from_str(game.store_link.clone().as_str()).unwrap();
            let mut headers = HeaderMap::new();
            headers.append(COOKIE, COOKIES.parse().unwrap());
            let offer_until = match make_request(http, &store_link, headers).await {
                Ok(response) => {
                    let Ok(html) = response.text().await else {
                        continue;
                    };
                    
                    let regex = Regex::new(r#"window.productcardData.cardProductPromoEndDate\s*=\s*\{\"date\":\"(\d{4}-\d{2}-\d{2})"#).unwrap();

                    let Some(captures) = regex.captures(&html) else {
                        log::error!("Couldn\'t find offer_until on page {store_link}");
                        continue;
                    };

                    NaiveDate::parse_from_str(&captures[1], "%Y-%m-%d").unwrap()
                },
                Err(e) => {
                    log::error!("Error getting page {}: {e}", store_link.as_str());
                    continue;
                }
            };
            
            free_games.push(Game {
                id: game.id,
                store: GameStore::Gog,
                title: game.title.clone(),
                identifier: make_identifier(game.title),
                url: game.store_link,
                original_price: game.price.base,
                offer_until,
                game_type: game.product_type,
            })
        }

        free_games
    }
}
