use std::sync::Arc;
use tokio_postgres::{Client, Row};
use utils::model::{Game, GameStore, GameType, PartialGame, PostedPlatform};

const MAX_RETRIES: u64 = 10;
const RETRY_DELAY: u64 = 5;

#[derive(Clone)]
pub(crate) struct Database {
    client: Arc<Client>,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let mut retries = 0;
        loop {
            match tokio_postgres::connect(url.as_str(), postgres::NoTls).await {
                Ok((client, connection)) => {
                    tokio::spawn(async move {
                        if let Err(e) = connection.await {
                            log::error!("Connection error: {e}");
                        }
                    });

                    log::info!("Connected to database.");

                    return Ok(Database {
                        client: Arc::new(client),
                    });
                }
                Err(e) => {
                    retries += 1;
                    log::error!("Could not connect to database (attempt: {retries}): {e}");
                    if retries >= MAX_RETRIES {
                        return Err(format!("Database connection could not be established after {MAX_RETRIES} tries: {e}").into());
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(RETRY_DELAY)).await;
        }
    }

    pub async fn add_game(&self, game: &Game) -> Result<(), tokio_postgres::Error> {
        let partial = PartialGame { id: game.id.clone(), store: game.store.clone() };
        if self.game_exists(&partial).await? {
            return Ok(());
        }

        let query = r#"
            INSERT INTO games
            (id, store, created_at, title, identifier, url, original_price, offer_until, game_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);"#;
        let now = chrono::Utc::now().date_naive();

        self.client.execute(
            query,
            &[
                &game.id,
                &game.store.to_string(),
                &now,
                &game.title,
                &game.identifier,
                &game.url,
                &game.original_price,
                &game.offer_until,
                &game.game_type.to_string(),
            ],
        )
            .await?;

        Ok(())
    }

    pub async fn remove_game(&self, game: &PartialGame) -> Result<(), tokio_postgres::Error> {
        let query = r#"DELETE FROM games WHERE id = $1 AND store = $2;"#;

        self.client.execute(query, &[&game.id, &game.store.to_string()])
            .await?;

        self.remove_posted(game).await?;

        Ok(())
    }

    pub async fn _remove_inactive_games(&self) -> Result<(), tokio_postgres::Error> {
        let query = r#"DELETE FROM games WHERE offer_until < CURRENT_DATE;"#;

        self.client.execute(query, &[]).await?;

        Ok(())
    }

    pub async fn game_exists(&self, game: &PartialGame) -> Result<bool, tokio_postgres::Error> {
        let query = r#"SELECT * FROM games WHERE id = $1 AND store = $2;"#;

        match self
            .client
            .query(query, &[&game.id, &game.store.to_string()])
            .await
        {
            Ok(rows) => Ok(!rows.is_empty()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_all_games(&self) -> Result<Vec<Game>, tokio_postgres::Error> {
        let query = r#"SELECT * FROM games"#;

        let mut games = Vec::new();

        let rows = self.client.query(query, &[]).await?;

        for row in rows {
            let game = row_to_game(&row)?;
            games.push(game);
        }

        Ok(games)
    }

    pub async fn get_active_games(&self) -> Result<Vec<Game>, tokio_postgres::Error> {
        let query = r#"SELECT * FROM games WHERE offer_until > CURRENT_DATE"#;

        let mut games = Vec::new();

        let rows = self.client.query(query, &[]).await?;

        for row in rows {
            let game = row_to_game(&row)?;
            games.push(game);
        }

        Ok(games)
    }

    pub async fn is_posted(&self, post_data: &PostedPlatform) -> Result<bool, tokio_postgres::Error> {
        let query = r#"SELECT * FROM platform_posts
            WHERE game_id = $1
            AND game_store = $2
            AND platform = $3;
        "#;

        match self
            .client
            .query(query, &[&post_data.game_id, &post_data.game_store.to_string(), &post_data.platform])
            .await
        {
            Ok(rows) => Ok(!rows.is_empty()),
            Err(e) => Err(e),
        }
    }

    pub async fn add_posted(&self, post_data: &PostedPlatform) -> Result<bool, tokio_postgres::Error> {
        let query = r#"INSERT INTO platform_posts
            (game_id, game_store, platform)
            VALUES ($1, $2, $3);"#;

        match self.client.execute(query, &[&post_data.game_id, &post_data.game_store.to_string(), &post_data.platform]).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub async fn remove_posted(&self, game: &PartialGame) -> Result<(), tokio_postgres::Error> {
        let query = r#"DELETE FROM platform_posts WHERE game_id = $1 AND game_store = $2;"#;

        self.client.execute(query, &[&game.id, &game.store.to_string()])
            .await?;

        Ok(())
    }
}

fn row_to_game(row: &Row) -> Result<Game, tokio_postgres::Error> {
    let store: String = row.try_get("store")?;
    let game_type: String = row.try_get("game_type")?;

    Ok(Game {
        id: row.try_get("id")?,
        store: GameStore::from(store),
        title: row.try_get("title")?,
        identifier: row.try_get("identifier")?,
        url: row.try_get("url")?,
        original_price: row.try_get("original_price")?,
        offer_until: row.try_get("offer_until")?,
        game_type: GameType::from(game_type),
    })
}
