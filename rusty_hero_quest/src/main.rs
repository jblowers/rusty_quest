mod game_state;
use game_state::GameState;
use warp::http::Method;
use warp::Filter;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;




#[cfg(test)]
mod tests;


// fn main() {

#[tokio::main]
async fn main() {    
/*****************************SERVER SECTION *******************/
// establish the in memory instances of games... 
// - need to create end point for creating a game
    // let mut games: HashMap<u32,GameState> = HashMap::new();
    let games: Arc<Mutex<HashMap<u32, GameState>>> = Arc::new(Mutex::new(HashMap::new()));


    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec![Method::GET, Method::POST]) // Adjust methods as needed
        .allow_headers(vec!["Content-Type"]);                           


    // // Create new game route
    // let gamestate_clone = gamestate.clone();
    let new_game_route = {
        let games = games.clone();
        warp::path!("game_state" / "new_game")
        .map(move || {
            let mut games = games.lock().unwrap();
            let new_id = games.len() as u32;
            let new_game_state = GameState::new_game(new_id);
            games.insert(new_id,new_game_state);
            let new_game_state = games.get(&new_id).cloned();
            warp::reply::json(&new_game_state)
        })
    };
    // // Game state route
    // let gamestate_clone = gamestate.clone();
    let game_state_id_route = {
        let games = games.clone();
        warp::path!("game_state" / u32)
        .map(move |game_id| {
            let games = games.lock().unwrap();
            let req_state = games.get(&game_id).cloned();
            // Logic to retrieve game state for the given game_id
            warp::reply::json(&req_state)
        })
    };
    // // Player route
    // // let gamestate_clone = gamestate.clone();
    // let player_route = warp::path!("players" / u32)
    //     .map(move |player_id| {
    //         let gamestate = gamestate_clone.lock().unwrap();
    //         // Logic to retrieve player information
    //         warp::reply::json(&*gamestate)
    //     });

    // Combine all routes
    let routes = new_game_route
        .or(game_state_id_route)
        .with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;


}


        /*
    // Player hand route
    // let gamestate_clone = gamestate.clone();
    let player_hand_route = warp::path!("players" / u32 / "hand")
        .map(move |player_id| {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve player's hand
            warp::reply::json(&*gamestate)
        });

    // Board route
    // let gamestate_clone = gamestate.clone();
    let board_route = warp::path("board")
        .map(move || {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve board state
            warp::reply::json(&*gamestate)
        });

    // Board spaces all route
    // let gamestate_clone = gamestate.clone();
    let board_spaces_all_route = warp::path!("board" / "spaces" / "all")
        .map(move || {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve all board spaces
            warp::reply::json(&*gamestate)
        });

    // Board space route
    // let gamestate_clone = gamestate.clone();
    let board_space_route = warp::path!("board" / "spaces" / u32)
        .map(move |space_id| {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve specific board space
            warp::reply::json(&*gamestate)
        });

    // Card route
    // let gamestate_clone = gamestate.clone();
    let card_route = warp::path!("cards" / u32)
        .map(move |card_id| {
            let gamestate = gamestate_clone.lock().unwrap();
            // Logic to retrieve specific card information
            warp::reply::json(&*gamestate)
        });
        */