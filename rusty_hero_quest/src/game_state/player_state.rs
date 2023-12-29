use super::card_collection;
use serde::{Serialize, Deserialize};


// Player data
#[derive(Debug,Serialize,Deserialize, PartialEq)]
pub struct Player {
    pub id: u32,
    pub name: String,
    position: u32,
    combat_skill: u8,
    defense_skill: u8,
    wounds: u8,
    equipment_combat_score: i32,
    equipment_defense_score: i32,
    ability_combat_score: i32,
    ability_defense_score: i32,
    pub hand: card_collection::CardCollection,
}
impl Player {
    pub fn new() -> Self {
        let mut hand = card_collection::CardCollection::new();
        Self {
            id: 0,
            name: "".to_string(),
            position: 0,
            combat_skill: 0,
            defense_skill: 0,
            wounds: 0,
            equipment_combat_score: 1,
            equipment_defense_score: 0,
            ability_combat_score: 0,
            ability_defense_score: 0,
            hand: hand,
        }
    }


    pub fn add_to_hand(&mut self,mut cardcoll: card_collection::CardCollection) {
        for card in cardcoll.get_cards_iterable() {            
            self.hand.add_card(card);
        }
    }
}