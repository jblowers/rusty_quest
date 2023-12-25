// use criterion::black_box;
// mod super::super::game_state;
// use super::super::game_state;
// use super::super::game_state::card_collection::*;
// use crate::tests::game_state_tests::CardState;
use crate::game_state;
use crate::game_state::card_collection;


#[test]
fn test_populate_with_fresh_cards() {    
    // let mut coll = card_collection::CardCollection::new();
    // coll.populate_self_with_fresh_cards(CARD_COUNT as u32);
    // assert_eq!(coll.cards.len(), CARD_COUNT,"Deck size should be 52 to start");
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
    assert_eq!(gs.discard.size(),0 as usize,"discard should be zero to start");
    let mut card = card_collection::Card {
        typ: card_collection::CardType::Good,
        value: 1,
        state: card_collection::CardState::InUse,
    };
    
    gs.discard_card(card);
    assert_eq!(gs.discard.size(), 1, "Discard should be 1 after discarding 1 card");


}

// #[test]
// fn test_draw_card() {

//     // let CARD_COUNT: usize = 52;
//     let mut coll = card_collection::CardCollection::new();//.populate_self_with_fresh_cards();
//     coll.populate_self_with_fresh_cards(CARD_COUNT as u32);
//     assert_eq!(coll.cards.len(), CARD_COUNT, "Deck size should be 52 to start");

//     // Draw a card and check if it exists
//     let drawn_card = black_box(coll.draw_card());
//     assert!(drawn_card.is_some(), "Expected to draw a card, but got None");

//     assert_eq!(coll.cards.len(), CARD_COUNT-1, "Deck size should decrease after drawing a card");
// }


