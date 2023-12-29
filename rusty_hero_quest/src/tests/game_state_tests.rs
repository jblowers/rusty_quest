// use criterion::black_box;
// use super::super::game_state::card_collection;
// mod game_state;

use super::super::game_state::card_collection;

use super::super::game_state;


const CARD_COUNT: u32 = 52;
const FIRST_CARD: CardCollection::Card = CardCollection::Card {
    typ: game_state::card_collection::CardType::Good,
    value: 1,
};


#[test]
fn test_draw_card() {
    let mut gs = game_state::GameState::new();
    assert_eq!(gs.cards_remaining_in_deck(),CARD_COUNT,"Should have all cards remaining in deck");

    // let card = gs.draw_card();
    // assert_eq!()
}


//     // let CARD_COUNT: usize = 52;
//     let mut coll = card_collection::CardCollection::new();//.populate_self_with_fresh_cards();
//     coll.populate_self_with_fresh_cards(CARD_COUNT as u32);
//     assert_eq!(coll.cards.len(), CARD_COUNT, "Deck size should be 52 to start");

//     // Draw a card and check if it exists
//     let drawn_card = black_box(coll.draw_card());
//     assert!(drawn_card.is_some(), "Expected to draw a card, but got None");

//     assert_eq!(coll.cards.len(), CARD_COUNT-1, "Deck size should decrease after drawing a card");
// }


