use regex::Regex;
use reqwest::{Client, Url};
use reqwest::header::HeaderMap;
use utils::model::Game;

mod steam;
mod epicgames;

pub use epicgames::EpicGamesStore;
pub use steam::SteamStore;
use utils::internal_api::InternalApi;

#[async_trait::async_trait]
pub(crate) trait Store {
    async fn get_games(&self, http: &Client, internal_api: &InternalApi) -> Vec<Game>;
}

async fn make_request(http: &Client, url: &Url, mut headers: HeaderMap) -> Result<reqwest::Response, reqwest::Error> {
    headers.append("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0".parse().unwrap());

    let request = http.get(url.clone()).headers(headers);

    match request.send().await {
        Ok(response) => {
            Ok(response)
        },
        Err(e) => Err(e)
    }
}

fn make_identifier(mut name: String) -> String {
    let identifier_regex = Regex::new(r"[^\p{L}\s]").unwrap();

    name = identifier_regex.replace_all(&name, "").to_string();
    name.replace(' ', "_")
}
