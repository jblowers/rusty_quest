
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
    fn endpoint(&self) -> &'static str {
        match self {
            GameAction::StartGame => "/start_game", // probably not valid yet
            GameAction::MoveWithSelectedCard => "/move_with_selected_card",
            GameAction::MoveWithGivenCard => "/move_with_given_card",
            GameAction::TakeEquipment => "/take_equipment",
            GameAction::UpgradeCombat => "/UpgradeCombat",
            GameAction::UpgradeDefense => "/UpgradeDefense",
            GameAction::EndTurn => "/end_turn",
            // ... other actions ...
        }
    }

    fn description(&self) -> &'static str {
        match self {
            GameAction::StartGame => "Begin a new game.",
            GameAction::MoveWithSelectedCard => "Move with selected card from hand. Should contain a Card Id",
            GameAction::MoveWithGivenCard => "Move with the current top card of the deck. Might contain a Card Id",
            GameAction::TakeEquipment => "Take equipment at the current space. If not an equipment space, return error.",
            GameAction::UpgradeCombat => "Discard cards to upgrade Combat. Should contain Card Id(s).",
            GameAction::UpgradeDefense => "Discard cards to upgrade Defense. Should contain Card Id(s).",
            GameAction::EndTurn => "End your turn",
            // ... other actions ...
        }
    }


}

#[derive(Serialize)]
struct ActionInfo {
    endpoint: &'static str,
    description: &'static str,
}

impl ActionInfo {
    fn new(action: GameAction) -> Self {
        Self {
            endpoint: action.endpoint(),
            description: action.description(),
        }
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
}

