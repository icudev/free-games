use actix_web::{HttpResponse, Responder, get, post, web::{Data, Json}, delete, HttpRequest};
use crate::database::Database;
use utils::model::{Game, PartialGame, PostedPlatform};

fn check_token(req: &HttpRequest) -> Result<(), HttpResponse> {
    let token = std::env::var("INTERNAL_API_AUTH_TOKEN").unwrap();
    
    let header = match req.headers().get("API-Token") {
        Some(header_value) => match header_value.to_str() {
            Ok(header) => header,
            Err(_) => return Err(HttpResponse::BadRequest().body("Invalid API-Token header"))
        },
        None => return Err(HttpResponse::Unauthorized().body("Missing API-Token header"))
    };
    
    if token.as_str() != header {
        return Err(HttpResponse::Unauthorized().body("Invalid API-Token"));
    }
    
    Ok(())
}


#[get("/")]
pub(crate) async fn index(db: Data<Database>, req: HttpRequest) -> impl Responder {
    log::debug!("GET /");

    if let Err(res) = check_token(&req) {
        return res;
    }

    match db.get_all_games().await {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/free")]
pub(crate) async fn get_free(db: Data<Database>, req: HttpRequest) -> impl Responder {
    log::debug!("GET /free");
    
    if let Err(res) = check_token(&req) {
        return res;
    }
    
    let games = db.get_active_games().await.unwrap_or_default();

    HttpResponse::Ok().json(games)
}

#[get("/game")]
pub(crate) async fn get_game(game: Json<PartialGame>, db: Data<Database>, req: HttpRequest) -> impl Responder {
    log::debug!("GET /game");

    if let Err(res) = check_token(&req) {
        return res;
    }

    match db.game_exists(&game).await {
        Ok(val) => HttpResponse::Ok().body(val.to_string()),
        Err(err) => {
            log::error!("GET /game failed: {err}");
            HttpResponse::BadRequest().finish()
        },
    }
}

#[delete("/game")]
pub(crate) async fn delete_game(game: Json<PartialGame>, db: Data<Database>, req: HttpRequest) -> impl Responder {
    log::debug!("DELETE /game");

    if let Err(res) = check_token(&req) {
        return res;
    }

    match db.remove_game(&game).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            log::error!("DELETE /game failed: {err}");
            HttpResponse::BadRequest().finish()
        },
    }
}

#[post("/game")]
pub(crate) async fn post_game(game: Json<Game>, db: Data<Database>, req: HttpRequest) -> impl Responder {
    log::debug!("POST /game");

    if let Err(res) = check_token(&req) {
        return res;
    }

    match db.add_game(&game).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            log::error!("POST /game failed: {err}");
            HttpResponse::BadRequest().finish()
        },
    }
}

#[get("/posted")]
pub(crate) async fn get_posted(posted_data: Json<PostedPlatform>, db: Data<Database>, req: HttpRequest) -> impl Responder {
    log::debug!("GET /posted");

    if let Err(res) = check_token(&req) {
        return res;
    }

    HttpResponse::Ok().body(db.is_posted(&posted_data).await.unwrap_or(true).to_string())
}

#[post("/posted")]
pub(crate) async fn post_posted(posted_data: Json<PostedPlatform>, db: Data<Database>, req: HttpRequest) -> impl Responder {
    log::debug!("POST /posted");

    if let Err(res) = check_token(&req) {
        return res;
    }

    match db.add_posted(&posted_data).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
