use std::collections::VecDeque;


pub const MAX_CARD_VALUE: u32 = 13;

pub struct CardCollection {
    cards: VecDeque<Card>,
}

impl CardCollection {
    pub fn new() -> Self {
        Self { cards: VecDeque::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    pub fn shuffle(&mut self) {

        // self.cards.shuffle();
    }

    // Add other helper functions as needed
    // For example, you might want functions to:
    // - Peek at the next card without removing it
    // - Remove a specific card from the collection
    // - Sort the cards based on a specific criteria
}


// Card data
pub struct Card {
    // id: u32,
    pub typ: CardType,
    pub value: u32,
}

pub enum CardType {
    Good,
    Bad,
}