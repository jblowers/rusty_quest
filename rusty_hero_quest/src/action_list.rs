use serde::{Serialize, Deserialize};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, Eq, PartialEq)]
pub enum GameAction {
    StartGame,
    MoveWithSelectedCard,
    MoveWithGivenCard,
    TakeEquipment,
    UpgradeCombat,
    UpgradeDefense,
    EndTurn,
    // ... other actions ...
} 

impl GameAction {
    fn endpoint(&self) -> String {
        match self {
            GameAction::StartGame => "/start_game".to_string(), // probably not valid yet
            GameAction::MoveWithSelectedCard => "/move_with_selected_card".to_string(),
            GameAction::MoveWithGivenCard => "/move_with_given_card".to_string(),
            GameAction::TakeEquipment => "/take_equipment".to_string(),
            GameAction::UpgradeCombat => "/UpgradeCombat".to_string(),
            GameAction::UpgradeDefense => "/UpgradeDefense".to_string(),
            GameAction::EndTurn => "/end_turn".to_string(),
            // ... other actions ...
        }
    }

    fn description(&self) -> String {
        match self {
            GameAction::StartGame => "Begin a new game.".to_string(),
            GameAction::MoveWithSelectedCard => "Move with selected card from hand. Should contain a Card Id".to_string(),
            GameAction::MoveWithGivenCard => "Move with the current top card of the deck. Might contain a Card Id".to_string(),
            GameAction::TakeEquipment => "Take equipment at the current space. If not an equipment space, return error.".to_string(),
            GameAction::UpgradeCombat => "Discard cards to upgrade Combat. Should contain Card Id(s).".to_string(),
            GameAction::UpgradeDefense => "Discard cards to upgrade Defense. Should contain Card Id(s).".to_string(),
            GameAction::EndTurn => "End your turn".to_string(),
            // ... other actions ...
        }
    }
    // needs to be updated when new GameActions are added
    fn iter_variants() -> &'static [GameAction] {
        & [
            GameAction::StartGame,
            GameAction::MoveWithSelectedCard,
            GameAction::MoveWithGivenCard,
            GameAction::TakeEquipment,
            GameAction::UpgradeCombat,
            GameAction::UpgradeDefense,
            GameAction::EndTurn,
        ]
    }


}

#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct GameActionList {
    actions: Vec<GameAction>,
}

impl GameActionList {
    pub fn add_action(&mut self, action: GameAction) {
        self.actions.push(action);
    }

    pub fn new() -> Self {
        GameActionList {
            actions : Vec::new(),
        }
    }

    pub fn contains(&self, action: GameAction) -> bool{
        self.actions.contains(&action)
    }
}

/*   ADDING ACTION_LIST_INFO structure */



// struct to make serializing to json easier for action_list_info end point

#[derive(Serialize,Deserialize,Clone)]
pub struct ActionInfoList {
    actions: HashMap<GameAction,ActionInfo>,
}
impl ActionInfoList {
    pub fn new() -> Self {
        let mut actions = HashMap::new();

        for &action in GameAction::iter_variants() {
            actions.insert(
                action,
                ActionInfo::new(action)
            );
        }
        Self { actions }
    }
    
}


#[derive(Serialize,Deserialize,Clone)]
struct ActionInfo {
    endpoint: String,
    description: String,
}

impl ActionInfo {
    fn new(action: GameAction) -> Self {
        Self {
            endpoint: action.endpoint(),
            description: action.description(),
        }
    }
}