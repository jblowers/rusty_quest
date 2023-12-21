mod game_state;
use game_state::GameState;
use warp::http::Method;
use warp::Filter;
use std::sync::{Arc, Mutex};





#[cfg(test)]
mod tests;


// fn main() {

#[tokio::main]
async fn main() {
    let mut gs = GameState::new();
    gs.shuffle_deck();
    gs.print_deck();

    let mut serialized_state = serde_json::to_string(&gs).unwrap();
    println!("{}", serialized_state);
    
    let mut deserial : GameState = serde_json::from_str(&serialized_state).unwrap();
    // deserial.print_deck();
    // gs.print_deck();

    
/*****************************SERVER SECTION *******************/


    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec![Method::GET, Method::POST]) // Adjust methods as needed
        .allow_headers(vec!["Content-Type"]);                           

        
    let gamestate = GameState::new();
    let gamestate = Arc::new(Mutex::new(gamestate));
    let gamestate_clone = gamestate.clone();
    let shuffle_deck_route = warp::path("shuffle_deck")
        .map(move || {
            let mut gamestate = gamestate_clone.lock().unwrap();
            gamestate.shuffle_deck();
            warp::reply::json(&*gamestate)
        }
    );
    let gamestate_clone = gamestate.clone();
// Define a route that sends the serialized game state
    let game_state_route = warp::path("game_state")
        .map(move || {
            let gamestate = gamestate_clone.lock().unwrap();
            warp::reply::json(&*gamestate)
        })
        .with(cors);

    warp::serve(game_state_route.or(shuffle_deck_route)).run(([0, 0, 0, 0], 3030)).await;

}
