use std::str::FromStr;
use chrono::{Datelike, NaiveDate};
use regex::Regex;
use reqwest::{Client, Url};
use reqwest::header::{HeaderMap, COOKIE};
use scraper::{Html, Selector};
use utils::internal_api::InternalApi;
use utils::model::{Game, GameStore, GameType, PartialGame};
use crate::stores::{make_identifier, make_request, Store};

pub struct SteamStore;

const STEAM_STORE_URL: &str =
    "https://store.steampowered.com/search/?cc=us&maxprice=free&specials=1";

// Cookies that allow the bot to view games that are 18+
const COOKIES: &str = "birthtime=788914801;lastagecheckage=1-January-1995;wants_mature_content=1;";

#[async_trait::async_trait]
impl Store for SteamStore {
    async fn get_games(&self, http: &Client, internal_api: &InternalApi) -> Vec<Game> {
        let search_result_selector = Selector::parse(r#"div[id="search_resultsRows"] a"#).unwrap();
        let game_discount_selector = Selector::parse(r#"div[class="discount_pct"]"#).unwrap();
        let steam_url_regex =
            Regex::new(r#"https://store.steampowered.com/app/(?<app_id>[0-9]+)/[ -~]+/"#).unwrap();

        let steam_search_url = Url::from_str(STEAM_STORE_URL).unwrap();

        let response = match make_request(http, &steam_search_url, HeaderMap::new()).await {
            Ok(response) => response,
            Err(_) => return vec![],
        };

        let game_urls = {
            let mut urls = vec![];

            let html = match response.text().await {
                Ok(html) => html,
                Err(_) => return vec![],
            };

            let document = Html::parse_document(&html);

            for element in document.select(&search_result_selector) {
                let discount_pct = match element.select(&game_discount_selector).next() {
                    Some(discount_pct) => discount_pct,
                    None => continue,
                };

                if discount_pct.inner_html().as_str() != "-100%" {
                    continue;
                }

                match element.attr("href") {
                    Some(href) => {
                        let mut string = href.to_string();
                        let char = if string.contains('?') { "&" } else { "?" };
                        string.push_str(format!("{char}cc=us").as_str());
                        urls.push(string);
                    }
                    None => continue,
                };
            }

            urls
        };

        let mut free_games = Vec::with_capacity(game_urls.len());

        for url in game_urls {
            let id = match steam_url_regex
                .captures(url.as_str())
                .map(|captures| captures[1].to_string()) 
            {
                Some(id) => id,
                None => continue,
            };

            let partial_game = PartialGame {
                id: id.clone(),
                store: GameStore::Steam,
            };

            match internal_api.get_game(&partial_game).await {
                Ok(false) => {
                    log::debug!("Game {id} does not exist.")
                },
                _ => {
                    log::debug!("Game {id} already exists, skipping.");
                    continue;
                }
            }

            let url = Url::from_str(url.as_str()).unwrap();
            if let Some(game) = parse_game_page(http, &url).await {
                free_games.push(game);
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }

        free_games
    }
}

async fn parse_game_page(http: &Client, url: &Url) -> Option<Game> {
    let steam_url_regex =
        Regex::new(r#"https://store.steampowered.com/app/(?<app_id>[0-9]+)/[ -~]+/"#).unwrap();

    let (game_url, game_id) = steam_url_regex
        .captures(url.as_str())
        .map(|captures| (captures[0].to_string(), captures[1].to_string()))?;

    let game_name_selector = Selector::parse(r#"div[id="appHubAppName"]"#).ok()?;
    let game_area_description_selector =
        Selector::parse(r#"div[id="game_area_description"] h2"#).ok()?;
    let game_original_price_selector =
        Selector::parse(r#"div[class="discount_original_price"]"#).ok()?;
    let game_offer_until_selector_quantity =
        Selector::parse(r#"p[class="game_purchase_discount_quantity "]"#).ok()?;

    let mut headers = HeaderMap::new();
    headers.append(COOKIE, COOKIES.parse().ok()?);

    let response = make_request(http, url, headers).await.ok()?;
    let document = Html::parse_document(&response.text().await.ok()?);

    let game_name = document.select(&game_name_selector).next()?.inner_html();
    let game_identifier = make_identifier(game_name.clone());
    let game_type = {
        let s = document.select(&game_area_description_selector)
            .next()?
            .inner_html();
        let split = s.split_whitespace();
        let last = split.last()?;

        match last.to_lowercase().as_str() {
            "game" => GameType::Game,
            "content" => GameType::Dlc,
            "software" => GameType::Software,
            "bundle" => GameType::Bundle,
            _ => GameType::Unknown,
        }
    };
    let game_original_price = document.select(&game_original_price_selector).next()?.inner_html();
    let game_offer_until = {
        let date_regex = Regex::new(r"(?<month>[a-zA-z]+) (?<day>[0-9]+)").unwrap();
        let this_year = chrono::Utc::now().year();
        let fmt = document.select(&game_offer_until_selector_quantity).next()?.inner_html();
        let (month, day) = match date_regex.captures(fmt.as_str()) {
            Some(captures) => (
                captures.name("month")?.as_str(),
                captures.name("day")?.as_str(),
            ),
            None => return None,
        };
        let mut offer_until = NaiveDate::parse_from_str(
            format!("{day} {month} {this_year}").as_str(),
            "%d %b %Y"
        ).ok()?;
        // We set the year of the offer to this year because steam doesn't add it. In the off-chance
        // that that offer would be in the past because of it we just add one to the current year.
        if offer_until < chrono::Utc::now().date_naive() {
            offer_until =
                NaiveDate::from_ymd_opt(this_year + 1, offer_until.month(), offer_until.day())?;
        };
        offer_until
    };

    Some(Game {
        id: game_id,
        store: GameStore::Steam,
        title: game_name,
        identifier: game_identifier,
        url: game_url,
        original_price: game_original_price,
        offer_until: game_offer_until,
        game_type,
    })
}
