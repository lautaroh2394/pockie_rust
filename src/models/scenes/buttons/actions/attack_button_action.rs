use crate::{enums::event::{BoardEvent, SceneEvent}, models::{fighter::Fighter, scenes::buttons::button::ButtonAction}};

pub struct AttackButtonAction {
    fighter: Fighter,
}

impl AttackButtonAction {
    pub fn new(fighter: Fighter) -> Self {
        Self {
            fighter
        }
    }
}

impl ButtonAction for AttackButtonAction {
    fn execute(&self, events: &mut Vec<SceneEvent>) {
        events.push(SceneEvent::BoardEvent(BoardEvent::BoardSelectVictim(self.fighter.clone())));
        println!("attack button clicked")
    }
}