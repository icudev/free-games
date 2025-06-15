mod model;

use std::str::FromStr;
use chrono::NaiveDate;
use reqwest::{Client, Url};
use reqwest::header::HeaderMap;
use utils::internal_api::InternalApi;
use utils::model::{Game, GameStore, PartialGame};
use crate::stores::{make_identifier, make_request, Store};
use crate::stores::epicgames::model::ApiResponse;

pub struct EpicGamesStore;

const EG_BASE_STORE_URL: &str = "https://store.epicgames.com/en-US/p";
const EG_API_URL: &str =
    "https://store-site-backend-static-ipv4.ak.epicgames.com/freeGamesPromotions";

#[async_trait::async_trait]
impl Store for EpicGamesStore {
    async fn get_games(&self, http: &Client, internal_api: &InternalApi) -> Vec<Game> {
        let url = Url::from_str(EG_API_URL).unwrap();
        let headers = HeaderMap::new();
        let res = make_request(http, &url, headers).await;

        match res {
            Ok(res) => {
                let text = res.text().await.unwrap_or_default();

                let api_response: ApiResponse = serde_json::from_str(text.as_str()).unwrap();
                let eg_games = api_response.data.catalog.search_store.elements;

                let mut free_games = Vec::with_capacity(eg_games.len());

                for game in eg_games {
                    let Some(ref promotions) = game.promotions else {
                        continue;
                    };

                    if promotions.promotional_offers.is_empty() ||
                        promotions.promotional_offers[0].promotional_offers.is_empty()
                    {
                        continue;
                    }

                    let current_promotional_offer =
                        &promotions.promotional_offers[0].promotional_offers[0];

                    let discount_setting =
                        &current_promotional_offer.discount_setting;

                    let skip_if = [
                        // Game is not currently free
                        game.price.total_price.discount_price > 0,
                        // Game is free without discount
                        game.price.total_price.original_price == 0,
                        game.status.as_str() != "ACTIVE",
                        discount_setting.discount_percentage != 0,
                    ];

                    if skip_if.iter().any(|x| *x) {
                        continue;
                    }

                    let partial_game = PartialGame {
                        id: game.id.clone(),
                        store: GameStore::EpicGames,
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

                    // TODO: Improve without closure
                    let build_game_url = |path|
                        { format!("{EG_BASE_STORE_URL}/{path}") };
                    let mut game_url = None;
                    if let Some(mappings) = game.catalog_ns.mappings {
                        for mapping in mappings {
                            if mapping.page_type.as_str() == "productHome" {
                                game_url = Some(build_game_url(mapping.page_slug));
                                break;
                            }
                        }
                    }
                    if game_url.is_none() {
                        for attribute in game.custom_attributes {
                            if attribute.key.as_str() == "com.epicgames.app.productSlug" {
                                game_url = Some(build_game_url(attribute.value));
                                break;
                            }
                        }
                        if game_url.is_none() {
                            continue;
                        }
                    }

                    let offer_until = {
                        let offer_until_fmt = current_promotional_offer.end_date.clone();
                        NaiveDate::parse_from_str(
                            offer_until_fmt.get(0..10).unwrap(), "%Y-%m-%d"
                        ).unwrap()
                    };

                    free_games.push(Game {
                        id: game.id,
                        store: GameStore::EpicGames,
                        title: game.title.clone(),
                        identifier: make_identifier(game.title.clone()),
                        url: game_url.unwrap(),
                        original_price: game.price.total_price.fmt_price.original_price.clone(),
                        offer_until,
                        game_type: game.offer_type,
                    })
                }

                free_games
            },
            Err(_) => vec![]
        }
    }
}
