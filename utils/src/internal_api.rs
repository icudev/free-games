use reqwest::header::HeaderMap;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::model::{Game, PartialGame, PostedPlatform};

type Error = Box<dyn std::error::Error>;

const MAX_RETRIES: u64 = 10;
const RETRY_DELAY: u64 = 5;

pub struct InternalApi {
    http_client: reqwest::Client,
    api_url: String,
    api_token: String,
}

impl InternalApi {
    pub fn new(api_url: String, api_token: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            api_url,
            api_token,
        }
    }

    fn build_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.api_url, endpoint)
    }
    
    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.append("API-Token", self.api_token.parse().unwrap());
        headers
    }

    async fn extract_item<D, B>(&self, endpoint: &str, body: Option<&B>) -> Result<D, Error>
    where D: DeserializeOwned, B: Serialize
    {
        let mut builder = self.http_client.get(self.build_url(endpoint)).headers(self.get_headers());
        if let Some(body) = body {
            builder = builder.json(body);
        }
        match builder.send().await {
            Ok(res) if res.status().is_success() => {
                let text = res.text().await?;
                Ok(serde_json::from_str(&text)?)
            },
            Ok(res) => {
                Err(
                    format!(
                        "extract_item failed on endpoint \"{}\": {}",
                        endpoint,
                        res.text().await?
                    ).into())
            },
            Err(e) => {
                Err(e.into())
            }
        }
    }

    async fn post_item<D>(&self, endpoint: &str, item: &D) -> Result<Response, Error>
    where D: Serialize
    {
        Ok(self.http_client.post(self.build_url(endpoint)).headers(self.get_headers()).json(item).send().await?)
    }

    async fn delete_item<D>(&self, endpoint: &str, item: D) -> Result<Response, Error>
    where D: Serialize
    {
        Ok(self.http_client.delete(self.build_url(endpoint)).headers(self.get_headers()).json(&item).send().await?)
    }

    pub async fn get_all_games(&self) -> Result<Vec<Game>, Error> {
        log::debug!("Getting all games");
        self.extract_item::<Vec<Game>, ()>("", None).await
    }

    pub async fn get_free_games(&self) -> Result<Vec<Game>, Error> {
        log::debug!("Getting all free games");
        self.extract_item::<Vec<Game>, ()>("free", None).await
    }

    pub async fn get_game(&self, game: &PartialGame) -> Result<bool, Error> {
        log::debug!("Check if item exists");
        self.extract_item("game", Some(game)).await
    }

    pub async fn post_game(&self, game: &Game) -> Result<Response, Error> {
        log::debug!("Posting game");
        self.post_item("game", game).await
    }

    pub async fn delete_game(&self, game: PartialGame) -> Result<Response, Error> {
        log::debug!("Delete game");
        self.delete_item("game", game).await
    }

    pub async fn is_posted(&self, posted_platform: &PostedPlatform) -> Result<bool, Error> {
        log::debug!("Check if game is posted");
        self.extract_item("posted", Some(posted_platform)).await
    }

    pub async fn post_posted(&self, posted_platform: &PostedPlatform) -> Result<Response, Error> {
        log::debug!("Post post");
        self.post_item("posted", posted_platform).await
    }
}

pub async fn wait_for_internal_api(internal_api: &InternalApi) -> Result<(), Error> {
    let mut retries = 0;
    loop {
        log::info!("Waiting for API...");

        match internal_api.get_all_games().await {
            Ok(_) => {
                log::info!("API ready");
                return Ok(());
            }
            Err(e) => {
                retries += 1;
                log::error!("API not available...");
                if retries >= MAX_RETRIES {
                    return Err(format!("Internal API not reachable: {e}").into());
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(RETRY_DELAY)).await;
    }
}
