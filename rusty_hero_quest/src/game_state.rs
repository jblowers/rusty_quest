pub mod card_collection;
use card_collection::Card;
use card_collection::CardCollection;
use std::collections::HashMap;
pub mod player_state;
use player_state::Player;
use serde::{Serialize, Deserialize};



// Equipment data
// #[derive(Debug)]
// TODO: This needs to be designed right (?)
#[derive(Debug,Serialize,Deserialize)]
struct Equipment {
    id: u32,
    position: u32,
    typ: EquipmentType,
    combat_score: i32,
    defense_score: i32,
}

// TODO: This needs to be designed right (?)
#[derive(Debug,Serialize,Deserialize)]
enum EquipmentType {
    MeleeWeapon,
    RangedWeapon,
    Armor,
}

// Ability data
// #[derive(Debug)]
// TODO: This needs to be designed right
#[derive(Debug,Serialize,Deserialize)]
struct Ability {
    id: u32,
    position: u32,
    typ: AbilityType,
    value: i32,
    duration: u32,
}

// TODO: This needs to be designed right
#[derive(Debug,Serialize,Deserialize)]
enum AbilityType {
    CombatBonus,
    DefenseBonus,
    SpecialAbility,
}

#[derive(Debug,Serialize,Deserialize)]
struct Enemy {
    name: String,
    attack_strength: i32,
    defense_strength: i32,
    equipment: Equipment,
}

#[derive(Debug,Serialize,Deserialize)]
struct Board {
    spaces: Vec<Space>,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
enum Space {
    Home,
    // Equipment(Equipment),
    // Ability(Ability),
    // Combat(Enemy),
}

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
        deck.populate_self_with_fresh_cards(52);
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

    pub fn cards_remaining_in_deck(&mut self)-> u32 {
        self.deck.size() as u32
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
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

    fn draw_card(&mut self, _player_id: u32) -> Option<Card> { // option allows for checking if empty
        return self.deck.draw_card();
    }

}

