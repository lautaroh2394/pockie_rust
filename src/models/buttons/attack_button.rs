use crate::models::fighter::Fighter;
use crate::models::position::Position;

use super::actions::attack_button_action::AttackButtonAction;
use super::button::Button;

pub struct AttackButton {}
impl AttackButton {
    pub fn new(parent_position: Position, fighter: Fighter) -> Button {
        let action = Box::new(AttackButtonAction::new(fighter));

        Button::new(
            parent_position,
            String::from("attack"),
            action,
        )
    }
}