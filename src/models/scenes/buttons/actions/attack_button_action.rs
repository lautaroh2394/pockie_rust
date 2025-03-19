use crate::{enums::event::{BoardEvent, SceneEvent}, global_events::push_global_event, models::{fighter::Fighter, scenes::buttons::button::ButtonAction}};

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
        push_global_event(SceneEvent::new_board_event(BoardEvent::BoardSelectVictim(self.fighter.clone())));
        println!("attack button clicked")
    }
}