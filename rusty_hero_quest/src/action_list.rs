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
    
    pub fn from_id(action_id: &u32) -> Option<Self> {
        match action_id {
            1 => Some(GameAction::StartGame),
            2 => Some(GameAction::MoveWithSelectedCard),
            3 => Some(GameAction::MoveWithGivenCard),
            4 => Some(GameAction::TakeEquipment),
            5 => Some(GameAction::UpgradeCombat),
            6 => Some(GameAction::UpgradeDefense),
            7 => Some(GameAction::EndTurn),
            // ... other actions ...
            _ => None,
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            GameAction::StartGame => 1,
            GameAction::MoveWithSelectedCard => 2,
            GameAction::MoveWithGivenCard => 3,
            GameAction::TakeEquipment => 4,
            GameAction::UpgradeCombat => 5,
            GameAction::UpgradeDefense => 6,
            GameAction::EndTurn => 7,
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

    pub fn to_action_info(&self) -> ActionInfo {
        match self {
            GameAction::StartGame => ActionInfo::new(GameAction::StartGame),
            GameAction::MoveWithSelectedCard => ActionInfo::new(GameAction::MoveWithSelectedCard),
            GameAction::MoveWithGivenCard => ActionInfo::new(GameAction::MoveWithGivenCard),
            GameAction::TakeEquipment => ActionInfo::new(GameAction::TakeEquipment),
            GameAction::UpgradeCombat => ActionInfo::new(GameAction::UpgradeCombat),
            GameAction::UpgradeDefense => ActionInfo::new(GameAction::UpgradeDefense),
            GameAction::EndTurn => ActionInfo::new(GameAction::EndTurn),
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
    id: u32,
}

impl ActionInfo {
    fn new(action: GameAction) -> Self {
        Self {
            endpoint: action.endpoint(),
            description: action.description(),
            id: action.id(),
        }
    }
}