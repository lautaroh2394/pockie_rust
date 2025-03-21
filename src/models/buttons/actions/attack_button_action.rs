use crate::{enums::{board_events::BoardEvent, event::SceneEvent}, global_events::push_global_event, models::{buttons::button::ButtonAction, fighter::Fighter}};

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
    fn execute(&self) {
        push_global_event(SceneEvent::board_event(BoardEvent::BoardSelectVictim(self.fighter.clone())));
        println!("attack button clicked")
    }
}