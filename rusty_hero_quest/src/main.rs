mod game_state;
use game_state::GameState;
mod action_list;
use action_list::GameActionList;
use action_list::ActionInfoList;
use action_list::GameAction;
use crate::game_state::player_state::Player;
use crate::game_state::card_collection::CardCollection;
use crate::game_state::card_collection;
use warp::http::Method;
use warp::Filter;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
// use warp::http::StatusCode;
use warp::reply::{self, Reply};



#[cfg(test)]
mod tests;

// static ACTIONLIST: Lazy<ActionInfoList> = Lazy::new(|| {
//     ActionInfoList::new()
// });

// fn main() {

#[tokio::main]
async fn main() {    
/*****************************SERVER SECTION *******************/
// establish the in memory instances of games... 
    // let mut games: HashMap<u32,GameState> = HashMap::new();
    let games: Arc<Mutex<HashMap<u32, GameState>>> = Arc::new(Mutex::new(HashMap::new()));

    // TODO: design how this will get updated after each route below
    let actionlist = ActionInfoList::new();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec![Method::GET, Method::POST]) // Adjust methods as needed
        .allow_headers(vec!["Content-Type"]);                           


    let action_info_route = {
        // need to respond with all the actions and their associated info (to be cached by client)
        warp::path!("actionlistinfo")
        .map(move || {
            warp::reply::json(&actionlist).into_response()
        })

    };
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

            // let mut new_action_list = GameActionList::new();
            // new_action_list.add_action(GameAction::StartGame);
            // actionlist_vec.insert(new_id,new_action_list);

            let new_game_state = games.get(&new_id).cloned();
            warp::reply::json(&new_game_state)
        })
    };

    // add a new player with a name
    let new_player_route = {
        let games = games.clone();
        warp::path!("game_state" / u32 / "new_player" / String)
        .map(move |game_id, player_name| {
            let mut games = games.lock().unwrap();
            let mut sel_game = games.get_mut(&game_id).expect("Game ID not found!");
            // let new_id = sel_game.players.len() as u32;
            let new_id = sel_game.add_player(Player::new_with_info(0,player_name));
            sel_game.add_action_for_player(new_id,GameAction::StartGame);
            
            
            warp::reply::json(&sel_game)
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

    let player_start_game_route = {
        let games = games.clone();
        warp::path!("game_state" / u32 / "player_actions" / u32 / "start_game")
        .map(move |game_id,player_id| {
            let mut games = games.lock().unwrap();

            if let Some(game_state) = games.get_mut(&game_id) {
                // first check if this action is even valid
                if let Some(actions) = game_state.actions_map.get(&player_id) {
                    if !actions.contains(GameAction::StartGame) {
                        // return some error here so the client knows they tried an invalid action.
                    }
                }
                // now do whatever we need to do to the player state or game state
                // in order to start a game.
                // such as... 
                //      set player position to HOME (space[0])
                //      populate the GameActionList for the player
                //      fill player's hand with drawn cards
                //      ... other things I'm sure.
                let mut drawn_cards = CardCollection::new();
                for _i in 0..4 {
                    let mut card = game_state.draw_card().expect("deck is empty!");
                    card.state = card_collection::CardState::PlayerHand;
                    drawn_cards.add_card(card);
                }
                if let Some(player_state) = game_state.players.get_mut(&player_id) {
                    player_state.add_to_hand(drawn_cards);
                }
            }
            // need to remove the StartGame action from the ActionList at this point
            let sel_game = games.get(&game_id).cloned();
            warp::reply::json(&sel_game)
        })
    };

    let cards_route = {
        let games = games.clone();
        warp::path!("game_state" / u32 / "cards")
        .map(move |game_id| {
            let games = games.lock().unwrap();
            
        if let Some(game_state) = games.get(&game_id) {
            let cards = game_state.get_deck().cards.clone();
            warp::reply::json(&cards).into_response()
        } else {
            // Card not found, reply with an appropriate error message
            warp::reply::with_status(
                "game not found",
                warp::http::StatusCode::NOT_FOUND,
            ).into_response()
        }
        })

    };


    // Cards route
    let cards_id_route = {
        let games = games.clone();
        warp::path!("game_state" / u32 / "cards" / u32)
        .map(move |game_id,card_id| {
            let games = games.lock().unwrap();
            // TODO: eventually needs to search through deck, discard, and player hands for cards; for now just search deck
            
            // want to go through deck.cards and find the one with the id 'card_id'
            // once found, reply with json represenation of that card object
            if let Some(game_state) = games.get(&game_id) {
                // Search through deck for the card with 'card_id'
                if let Some(card) = game_state.get_deck().cards.iter().find(|&c| c.id == card_id as i32) {
                    // Card found, reply with JSON representation
                    warp::reply::json(&card).into_response()
                } else {
                    // Card not found, reply with an appropriate error message
                    warp::reply::with_status(
                        "Card not found",
                        warp::http::StatusCode::NOT_FOUND,
                    ).into_response()
                }
            } else {
                // Game not found, reply with an appropriate error message
                warp::reply::with_status(
                    "Game not found",
                    warp::http::StatusCode::NOT_FOUND,
                ).into_response()
            }
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
        .or(cards_id_route)
        .or(cards_route)
        .or(action_info_route)
        .or(new_player_route)
        .or(player_start_game_route)
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