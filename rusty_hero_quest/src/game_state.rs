mod card_collection;
use card_collection::Card;
use card_collection::CardType;
use std::collections::HashMap;



// Player data
struct Player {
    id: u32,
    name: String,
    position: u32,
    combat_skill: u8,
    defense_skill: u8,
    wounds: u8,
    equipment_combat_score: i32,
    equipment_defense_score: i32,
    ability_combat_score: i32,
    ability_defense_score: i32,
    hand: Vec<Card>,
    action_points: u8,
}


// Equipment data
// #[derive(Debug)]
struct Equipment {
    id: u32,
    position: u32,
    typ: EquipmentType,
    combat_score: i32,
    defense_score: i32,
}

enum EquipmentType {
    MeleeWeapon,
    RangedWeapon,
    Armor,
}

// Ability data
// #[derive(Debug)]
struct Ability {
    id: u32,
    position: u32,
    typ: AbilityType,
    value: i32,
    duration: u32,
}

enum AbilityType {
    CombatBonus,
    DefenseBonus,
    SpecialAbility,
}

// Board data
// #[derive(Debug)]
struct Board {
    spaces: Vec<Space>,
}

// #[derive(Debug)]
#[derive(Clone)]
enum Space {
    Empty,
    // Occupied(Player),
    // Equipment(Equipment),
    // Ability(Ability),
}

// Game state
pub struct GameState {
    players: HashMap<u32, Player>,
    board: Board,
    deck: Vec<Card>,
    discard: Vec<Card>,
    // cards can also exist in player's hand... Need to track here or no?
    active_player_id: u32,
    turn: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: HashMap::new(),
            board: Board {
                spaces: vec![Space::Empty; 100],
            },
            deck: Vec::new(),
            discard: Vec::new(),
            active_player_id: 0,
            turn: 0,
        }
    }

    fn add_player(&mut self, mut player: Player) {
        let id = self.players.len() as u32 + 1; // id is always the next incremented player value; 
        // TODO: will need to be addressed if players quit
        player.id = id;
        self.players.insert(id,player);
    }

    fn draw_card(&mut self, _player_id: u32) -> Card {
        // need a Deck struct
        return Card { typ: CardType::Good, value: 0};
    }

    fn generate_fresh_deck_of_cards(card_count: u32) -> Vec<Card> {
        let mut deck = Vec::new();
        let count_each_type = card_count / card_collection::MAX_CARD_VALUE / 2; // 52 by 13 by 2 is 2 cards for each value/type
        for _iteration in 1..count_each_type {
            // should hit this twice
            for i in 1..card_collection::MAX_CARD_VALUE {
                let good_card = Card {typ: CardType::Good, value: i};
                let bad_card = Card {typ: CardType::Bad, value: i};
                deck.push(good_card);
                deck.push(bad_card);               
            }
        }

        return deck;
    }
}

