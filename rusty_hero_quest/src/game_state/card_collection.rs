use std::collections::VecDeque;
use rand::seq::SliceRandom; // Import the trait for shuffle
use rand::thread_rng;       // Import thread_rng for a random number generator



pub const MAX_CARD_VALUE: u32 = 13;

pub struct CardCollection {
    pub cards: VecDeque<Card>,
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

    pub fn get_cards_iterable(&mut self) -> Vec<Card> {
        // let coll_as_vec: Vec<Card> = self.cards.to_owned();
        let coll_as_vec: Vec<Card> = self.cards.iter().cloned().collect::<Vec<Card>>();
        return coll_as_vec;
        // return self.cards.drain(..).collect(); // this might empty the internal cards list... if I just wanted to view them, might be a problem
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng(); // Create a random number generator
        // must copy into a vec, do the shuffle, then move back into a VecDeque
        let mut cards_vec: Vec<_> = self.cards.drain(..).collect();
        cards_vec.shuffle(&mut rng);
        self.cards = cards_vec.into();
    }
    pub fn populate_self_with_fresh_cards(&mut self, card_count: u32) {        
        let mut deck = Vec::new();
        let count_each_type = card_count / MAX_CARD_VALUE / 2; // 52 by 13 by 2 is 2 cards for each value/type
        for _iteration in 0..count_each_type {
            // should hit this twice
            for i in 1..MAX_CARD_VALUE + 1 {
                let good_card = Card {typ: CardType::Good, value: i};
                let bad_card = Card {typ: CardType::Bad, value: i};
                deck.push(good_card);
                deck.push(bad_card);               
            }
        }

        self.cards = deck.into();
    }
    
    pub fn print_collection_contents(&mut self) {
        println!("contents:\n\tValue:\tType:");
        for card in &self.cards {
            // let typ = card.typ;
            let typeStr = if let CardType::Good = card.typ {
                "Good"
            } else {
                "Bad"
            };
            println!("\t {}\t{}",card.value,typeStr);
        }
    }
    // Add other helper functions as needed
    // - Peek at the next card without removing it
    // - Remove a specific card from the collection
    // - Sort the cards based on a specific criteria
}


// Card data
#[derive(Debug)]
pub struct Card {
    // id: u32,
    pub typ: CardType,
    pub value: u32,
}
impl Clone for Card {
    fn clone(&self) -> Self {
        Card {
            typ: self.typ.clone(),
            value: self.value,
        }
    }
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ && self.value == other.value
    }
}

#[derive(Debug,Copy)]
pub enum CardType {
    Good,
    Bad,
}
impl PartialEq for CardType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CardType::Good, CardType::Good) => true,
            (CardType::Bad, CardType::Bad) => true,
            _ => false,
        }
    }
}

impl Clone for CardType {
    fn clone(&self) -> Self {
        match *self {
            CardType::Good => CardType::Good,
            CardType::Bad => CardType::Bad,
        }
    }
}






