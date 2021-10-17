#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::State;
use serde::{Deserialize, Serialize};
use log::{info};

use std::sync::Mutex;
use monopoly::{game};

struct AppState {
    game: Mutex<game::Game>
}

#[derive(Deserialize)]
struct DiceRoll {
    dice1: u32,
    dice2: u32
}

#[derive(Serialize)]
struct GameResponse<'a> {
    message: &'a str,
    next_player: &'a str
}

#[get("/ping")]
fn ping() -> String {
    format!("pong")
}

#[post("/roll-dice", format="application/json", data="<input>")]
/// Roll the dice in an established game.
// Expects a request like: 
// curl -X POST -H "Content-type: application/json" \
//      -d '{"dice_roll": {"dice1": 3, "dice2": 2}}' \
//      http://127.0.0.1:8000/roll-dice
fn roll_dice(input: String,
                   app_state: State<AppState>) -> String {
    //info!("Got Dice: {}", input);
    let dice_roll: DiceRoll = serde_json::from_str(&input).unwrap();

    let dice = game::Dice::new(dice_roll.dice1, dice_roll.dice2);
    let g = app_state.game.lock().unwrap();
    g.go(dice);

    let p = g.players().get(g.active_player()).unwrap().borrow();
    let gr = GameResponse {
        message: &g.journal(),
        next_player: p.name()
    };
    serde_json::to_string(&gr).unwrap()
}


fn main() {
    let app_state = AppState { 
        game: Mutex::new(game::init(vec![
                "Hannah".to_string(),
                "Daniel".to_string(),
                "Daddy".to_string(),
        ]))
    };

    let default = rocket_cors::CorsOptions::default().to_cors()
        .expect("error creating CORS fairing");

    rocket::ignite()
        .manage(app_state)
        .attach(default)
        .mount("/", routes![ping, roll_dice])
        .launch();
}
