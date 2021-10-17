use actix_cors::Cors;
use actix_web::{web, get, post, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};
use log::{info};

use std::sync::Mutex;
use monopoly::{game};

struct AppState {
    num: Mutex<u32>,
//    game: Mutex<game::Game>
}

#[derive(Serialize, Deserialize, Debug)]
struct DiceRoll {
    dice1: u32,
    dice2: u32
}

#[get("/ping")]
async fn ping() -> String {
    format!("pong")
}

#[post("/roll-dice")]
/// Roll the dice in an established game.
// Expects a request like: 
// curl -X POST -H "Content-type: application/json" \
//      -d '{"dice_roll": {"dice1": 3, "dice2": 2}}' \
//      http://127.0.0.1:8000/roll-dice
async fn roll_dice(app_state: web::Data<AppState>,
                   dice_roll: web::Json<DiceRoll>) -> HttpResponse {
    info!("Got {:?}", dice_roll);

//    let dice = game::Dice::new(dice_roll.dice1, dice_roll.dice2);
//    let g = app_state.game.lock().unwrap();
//    info!("Active player: {:?}", g.active_player());
//    g.go(dice);
//    info!("Active player: {:?}", g.active_player());
//
    let mut n = app_state.num.lock().unwrap();
    info!("num b4: {}", n);
    *n += 1;
    info!("num aftr: {}", n);
    // TODO: pass back all game information
    let dice = DiceRoll { dice1: *n, dice2: 2};
    let response = format!("{}",
                           serde_json::to_string(&dice).unwrap());

    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let app_data = web::Data::new(AppState { 
        num: Mutex::new(0),
//        game: Mutex::new(game::init(vec![
//                "Hannah".to_string(),
//                "Daniel".to_string(),
//                "Daddy".to_string(),
//        ]))
    });
    HttpServer::new(move || {
        // TODO: Only for local testing
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // local react app
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
//            .app_data(app_data)
            .app_data(app_data.clone())
//            .app_data(web::Data::new(AppState {
//                num: Mutex::new(0),
//                game: Mutex::new(game::init(
//                        vec!["Hannah".to_string(), "Daniel".to_string(),
//                             "Daddy".to_string()]))
//            }))
            .service(ping)
            .service(roll_dice)
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}
