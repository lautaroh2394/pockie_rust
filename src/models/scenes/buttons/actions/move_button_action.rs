use crate::{enums::event::{BoardEvent, SceneEvent}, models::{fighter::Fighter, scenes::buttons::button::ButtonAction}};

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
    fn execute(&self, events: &mut Vec<SceneEvent>) {
        events.push(SceneEvent::BoardEvent(BoardEvent::BoardSelectMove(self.fighter.clone())));
        println!("move button clicked")
    }
}