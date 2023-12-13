mod game_state;
use game_state::GameState;
// use game_state;
// mod /card_collection;
// use game_state/card_collection;

#[cfg(test)]
mod tests;


fn main() {

    let mut gs = GameState::new();
    gs.shuffle_deck();
    gs.print_deck();

    // println!("Hello, world!");
}
