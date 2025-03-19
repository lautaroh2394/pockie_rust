use crate::{enums::event::{BoardEvent, SceneEvent}, global_events::push_global_event, models::{fighter::Fighter, scenes::buttons::button::ButtonAction}};

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
        push_global_event(SceneEvent::new_board_event(BoardEvent::BoardSelectMove(self.fighter.clone())));
        println!("move button clicked")
    }
}