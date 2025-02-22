use crate::models::position::Position;

use super::button::Button;
use super::actions::move_button_action::MoveButtonAction;

pub struct MoveButton {}
impl MoveButton {
    pub fn new(parent_position: Position) -> Button {
        let action = Box::new(MoveButtonAction {});

        Button::new(
            parent_position,
             String::from("move"),
            action,
        )
    }
}