pub mod card_collection;
use card_collection::Card;
use card_collection::CardCollection;
use card_collection::CardState;
use std::collections::HashMap;
pub mod player_state;
use player_state::Player;
use serde::{Serialize, Deserialize};


// Game state
#[derive(Debug,Serialize,Deserialize)]
pub struct GameState {
    players: HashMap<u32, Player>,
    board: Board,
    deck: CardCollection,
    discard: CardCollection,
    // cards can also exist in player's hand... Need to track here or no?
    active_player_id: u32,
    turn: u32,
}

impl GameState {
    pub fn new() -> Self {
        
        let mut deck = CardCollection::new();
        deck.populate_self_with_fresh_cards(52, CardState::Deck);
        GameState {
            players: HashMap::new(),
            board: Board {
                spaces: vec![Space::Home],
            },
            deck,
            discard: CardCollection::new(),
            active_player_id: 0,
            turn: 0,
        }
    }

    pub fn get_deck(&mut self) -> &CardCollection {
        &self.deck
    }

    pub fn get_discard(&mut self) -> &CardCollection {
        &self.discard
    }

    pub fn cards_remaining_in_deck(&mut self)-> u32 {
        self.deck.size() as u32
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }

    pub fn replenish_deck_from_discard(&mut self) {
        let discarded_cards = self.discard.draw_all();
        self.deck.add_cards(discarded_cards);
        self.shuffle_deck();
    }

    pub fn print_deck(&mut self) {
        self.deck.print_collection_contents();
    }

    fn add_player(&mut self, mut player: Player) {
        let id = self.players.len() as u32 + 1; // id is always the next incremented player value; 
        // TODO: will need to be addressed if players quit
        player.id = id;
        self.players.insert(id,player);
    }

    pub fn draw_card(&mut self, _player_id: u32) -> Option<Card> { // option allows for checking if empty
        return self.deck.draw_card();
    }

    pub fn discard_card(&mut self, mut card: Card) {
        card.state = CardState::Discard;
        self.discard.add_card(card);
    }

    // pub fn flip_top_card(&mut self) -> Option<Card> {
    //     let mut card = self.draw_card();
    //     if card.is_empty() {
    //         // handle this error
    //     }
    //     card.unwrap().state = CardCollection::CardState::InUse;

}
impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.players == other.players
            && self.board == other.board
            && self.deck == other.deck
            && self.discard == other.discard
            && self.active_player_id == other.active_player_id
            && self.turn == other.turn
    }
}




// ************************************ //
// *************** Aux defines ******** //



#[derive(Debug,Serialize,Deserialize, PartialEq)]
struct Board {
    spaces: Vec<Space>,
}

#[derive(Clone,Debug,Serialize,Deserialize, PartialEq)]
enum Space {
    Home,
    Equipment(Equipment),
    Ability(Ability),
    Combat(Enemy),
}

// TODO: This needs to be designed right (?)
#[derive(Clone,Debug,Serialize,Deserialize, PartialEq)]
struct Equipment {
    id: u32,
    position: u32,
    typ: EquipmentType,
    combat_score: i32,
    defense_score: i32,
}

// TODO: This needs to be designed right (?)
#[derive(Clone,Debug,Serialize,Deserialize, PartialEq)]
enum EquipmentType {
    MeleeWeapon,
    RangedWeapon,
    Armor,
}

// TODO: This needs to be designed right
#[derive(Clone,Debug,Serialize,Deserialize, PartialEq)]
struct Ability {
    id: u32,
    position: u32,
    typ: AbilityType,
    value: i32,
    duration: u32,
}

// TODO: This needs to be designed right
#[derive(Clone,Debug,Serialize,Deserialize, PartialEq)]
enum AbilityType {
    CombatBonus,
    DefenseBonus,
    SpecialAbility,
}

// TODO: This needs to be designed right
#[derive(Clone,Debug,Serialize,Deserialize, PartialEq)]
struct Enemy {
    name: String,
    attack_strength: i32,
    defense_strength: i32,
    equipment: Equipment,
}
