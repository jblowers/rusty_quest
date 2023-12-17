pub mod card_collection;
use card_collection::Card;
use card_collection::CardType;
use card_collection::CardCollection;
use std::collections::HashMap;
pub mod player_state;
use player_state::Player;


// Equipment data
// #[derive(Debug)]
// TODO: This needs to be designed right (?)
struct Equipment {
    id: u32,
    position: u32,
    typ: EquipmentType,
    combat_score: i32,
    defense_score: i32,
}

// TODO: This needs to be designed right (?)
enum EquipmentType {
    MeleeWeapon,
    RangedWeapon,
    Armor,
}

// Ability data
// #[derive(Debug)]
// TODO: This needs to be designed right
struct Ability {
    id: u32,
    position: u32,
    typ: AbilityType,
    value: i32,
    duration: u32,
}

// TODO: This needs to be designed right
enum AbilityType {
    CombatBonus,
    DefenseBonus,
    SpecialAbility,
}

struct Enemy {
    name: String,
    attack_strength: i32,
    defense_strength: i32,
    equipment: Equipment,
}

// Board data
// #[derive(Debug)]
struct Board {
    spaces: Vec<Space>,
}

// #[derive(Debug)]
#[derive(Clone)]
enum Space {
    Home,
    // Occupied(Player),
    Equipment(Equipment),
    Ability(Ability),
    Combat(Enemy),
}

// Game state
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
                spaces: vec![Space::Empty; 100],
            },
            deck,
            discard: CardCollection::new(),
            active_player_id: 0,
            turn: 0,
        }
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

