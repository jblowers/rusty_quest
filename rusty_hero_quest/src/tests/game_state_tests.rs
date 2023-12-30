use crate::game_state;
use crate::game_state::card_collection;



const CARD_COUNT: u32 = 52;


#[test]
fn test_draw_card() {
    let mut gs = game_state::GameState::new();
    assert_eq!(gs.cards_remaining_in_deck(),CARD_COUNT,"Should have all cards remaining in deck");
}


#[test]
fn serialize_deserialize_game_state() {
    
    let mut gs = game_state::GameState::new();
    gs.shuffle_deck();
    // gs.print_deck();

    let mut serialized_state = serde_json::to_string(&gs).unwrap();
    let mut deserial : game_state::GameState = serde_json::from_str(&serialized_state).unwrap();
    assert_eq!(&gs,&deserial,"Game states should be the same before and after serialize/deserialize");
}

#[test]
fn flip_top_card_to_discard() {
    let mut gs = game_state::GameState::new();
    // deck should still be in order
    // let card = gs.flip_top_card();

}

#[test]
fn add_card_to_discard_pile() {
    let mut gs = game_state::GameState::new();
    assert_eq!(gs.get_discard().size(),0 as usize,"discard should be zero to start");
    let mut card = card_collection::Card {
        id: 1,
        typ: card_collection::CardType::Good,
        value: 1,
        state: card_collection::CardState::InUse,
    };
    
    gs.discard_card(card);
    assert_eq!(gs.get_discard().size(), 1, "Discard should be 1 after discarding 1 card");
}

#[test]
fn shuffle_discard_into_deck() {
    let mut gs = game_state::GameState::new();
    for i in 0..gs.get_deck().size() {
        let card = gs.draw_card(0);
        gs.discard_card(card.unwrap());
    }
    assert_eq!(gs.get_deck().size(),0,"deck size should be zero");
    assert_eq!(gs.get_discard().size(),52,"discard size should be 52");
    gs.replenish_deck_from_discard();
    assert_eq!(gs.get_deck().size(),52, "Deck size should be 52 again");
    assert_eq!(gs.get_discard().size(),0,"discard size should be 0 again");
}



