use criterion::black_box;
use super::super::game_state::card_collection;


const CARD_COUNT: usize = 52;


#[test]
fn test_populate_with_fresh_cards() {    
    let mut coll = card_collection::CardCollection::new();
    coll.populate_self_with_fresh_cards(CARD_COUNT as u32, card_collection::CardState::InUse);
    assert_eq!(coll.cards.len(), CARD_COUNT,"Deck size should be 52 to start");
}

#[test]
fn test_draw_card() {
    let mut coll = card_collection::CardCollection::new();
    coll.populate_self_with_fresh_cards(CARD_COUNT as u32, card_collection::CardState::Deck);
    assert_eq!(coll.cards.len(), CARD_COUNT, "Deck size should be 52 to start");

    // Draw a card and check if it exists
    let drawn_card = black_box(coll.draw_card());
    assert!(drawn_card.is_some(), "Expected to draw a card, but got None");

    assert_eq!(coll.cards.len(), CARD_COUNT-1, "Deck size should decrease after drawing a card");
}


#[test]
fn test_is_empty() {
    let mut coll = card_collection::CardCollection::new();
    assert_eq!(coll.cards.len(), 0,"Cards in a new collection should be 0");
    assert!(coll.is_empty());

}


#[test]
fn test_add_card() {
    let mut coll = card_collection::CardCollection::new();
    let good = card_collection::CardType::Good;
    let val = 10;
    coll.add_card(card_collection::Card {typ:good, value: val, state: card_collection::CardState::InUse});
    assert_eq!(coll.cards.len(), 1, "Expect only 1 card in the deck");
    // assert_eq!(coll.cards[0].typ,good);
    assert_eq!(coll.cards[0].value,val);
}

#[test]
fn test_shuffle_randomness() {
    for _ in 0..1000 {
        let mut deck = create_test_collection();
        deck.shuffle();
        assert_ne!(deck.get_cards_iterable(), create_test_collection().get_cards_iterable()); // Decks shouldn't be identical
    }
}

fn create_test_collection() -> card_collection::CardCollection {
    let mut coll = card_collection::CardCollection::new();
    coll.populate_self_with_fresh_cards(CARD_COUNT as u32, card_collection::CardState::InUse);
    return coll;
}

#[test]
fn test_shuffle_card_presence() {
    let mut deck = create_test_collection();
    deck.shuffle();
    for card in create_test_collection().cards {
        assert!(deck.cards.contains(&card)); // All cards should be present
    }
}


#[test]
fn test_peek_top_card() {
    let mut coll = create_test_collection();
    let peeked_card = {
        let opt = coll.peek_top_card();
        assert!(opt.is_some(), "Top card should always be there because we created a test collection");
        opt.unwrap().clone()
    };
    let comp_card = coll.cards.pop_front().unwrap();//.unwrap();
    assert_eq!(peeked_card,comp_card, "peeked card should match the actual top card of discard in gs.");
}

#[test]
fn test_draw_all() {
    let mut coll = create_test_collection();
    let drawn_cards = coll.draw_all();
    assert_eq!(drawn_cards.len(),CARD_COUNT,"size of drawn cards should be same as a full deck");
    assert_eq!(coll.size(),0,"deck size should now be zero");
}