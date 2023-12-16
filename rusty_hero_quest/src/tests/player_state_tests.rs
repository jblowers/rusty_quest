// use criterion::black_box;
use super::super::game_state::player_state;
use super::super::game_state::card_collection;

const CARDS_DRAWN: u32 = 5;

#[test]
fn test_add_to_hand() {    
    let mut coll = card_collection::CardCollection::new();
    coll.populate_self_with_fresh_cards(52);
    let mut drawn_cards = card_collection::CardCollection::new();
    for i in 0..CARDS_DRAWN {
        let cardd = coll.draw_card();
        if cardd.is_some() {
            drawn_cards.add_card(cardd.unwrap());
        }
    }
    assert_eq!(drawn_cards.size(),CARDS_DRAWN as usize,"should have drawn 5 cards");

    let mut player = player_state::Player::new();
    assert_eq!(player.hand.size(),0, "Player hand should start as empty, len == 0");

    
    player.add_to_hand(drawn_cards);
    assert_eq!(player.hand.size(),CARDS_DRAWN as usize,"Didn't add the right number of cards to hand.");
}
