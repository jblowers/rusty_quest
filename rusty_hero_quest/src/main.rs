mod game_state;
use game_state::GameState;


#[cfg(test)]
mod tests;


fn main() {

    let mut gs = GameState::new();
    gs.shuffle_deck();
    gs.print_deck();

    // println!("Hello, world!");
}
