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
    // let mut gs = GameState::new();
    // gs.shuffle_deck();
    // gs.print_deck();

    // let mut serialized_state = serde_json::to_string(&gs).unwrap();
    // println!("{}", serialized_state);
    
    // let mut deserial : GameState = serde_json::from_str(&serialized_state).unwrap();
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


    // Game state route
    let gamestate_clone = gamestate.clone();
    let game_state_route = warp::path!("game_state")
        .map(move || {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve game state for the given game_id
            warp::reply::json(&*gamestate)
        });
    // // Game state route
    // let gamestate_clone = gamestate.clone();
    // let game_state_route = warp::path!("game_state" / u32)
    //     .map(move |game_id| {
    //         let gamestate = gamestate_clone.lock().unwrap();
    //         // Logic to retrieve game state for the given game_id
    //         warp::reply::json(&*gamestate)
    //     });

    // Player route
    let gamestate_clone = gamestate.clone();
    let player_route = warp::path!("players" / u32)
        .map(move |player_id| {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve player information
            warp::reply::json(&*gamestate)
        });

    // Player hand route
    let gamestate_clone = gamestate.clone();
    let player_hand_route = warp::path!("players" / u32 / "hand")
        .map(move |player_id| {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve player's hand
            warp::reply::json(&*gamestate)
        });

    // Board route
    let gamestate_clone = gamestate.clone();
    let board_route = warp::path("board")
        .map(move || {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve board state
            warp::reply::json(&*gamestate)
        });

    // Board spaces all route
    let gamestate_clone = gamestate.clone();
    let board_spaces_all_route = warp::path!("board" / "spaces" / "all")
        .map(move || {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve all board spaces
            warp::reply::json(&*gamestate)
        });

    // Board space route
    let gamestate_clone = gamestate.clone();
    let board_space_route = warp::path!("board" / "spaces" / u32)
        .map(move |space_id| {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve specific board space
            warp::reply::json(&*gamestate)
        });

    // Card route
    let gamestate_clone = gamestate.clone();
    let card_route = warp::path!("cards" / u32)
        .map(move |card_id| {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve specific card information
            warp::reply::json(&*gamestate)
        });

    // Combine all routes
    let routes = game_state_route
        .or(player_route)
        .or(player_hand_route)
        .or(board_route)
        .or(board_spaces_all_route)
        .or(board_space_route)
        .or(card_route)
        .with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;


}
