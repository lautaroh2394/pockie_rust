use crate::{enums::{board_events::BoardEvent, event::SceneEvent}, global_events::push_global_event, models::{buttons::button::ButtonAction, fighter::Fighter}};

pub struct MoveButtonAction {
    fighter: Fighter,
}

impl MoveButtonAction {
    pub fn new(fighter: Fighter) -> Self {
        Self {
            fighter
        }
    }
}

impl ButtonAction for MoveButtonAction {
    fn execute(&self) {
        push_global_event(SceneEvent::board_event(BoardEvent::BoardSelectMove(self.fighter.clone())));
        println!("move button clicked")
    }
}