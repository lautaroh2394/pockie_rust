use macroquad::{color::GRAY, shapes::draw_rectangle};

use crate::{enums::event::SceneEvent, models::{fighter::Fighter, position::Position}, traits::game_object::GameObject};

use super::buttons::button::Button;

pub struct CharacterMenu {
    position: Position,
    options: Vec<Button>,

}

impl CharacterMenu {
    pub fn new_for_fighter(fighter: &Fighter) -> Self {
        CharacterMenu {
            position: fighter.get_pos().clone(),
            options: vec![
                Button::new(fighter.get_pos().clone(), String::from("attack")),
            ]
        }
    }
}

impl GameObject<SceneEvent> for CharacterMenu {
    fn draw(&self){
        draw_rectangle(self.get_x(), self.get_y(),  self.get_width(), self.get_height(), GRAY);
    }

    fn click_action(&mut self, _: &Position, _: &mut Vec<SceneEvent>){
        // todo - handle options
    }
    
    fn get_pos(&self) -> &Position {
        &self.position
    }

    fn click(&mut self, position: &Position, events: &mut Vec<SceneEvent>) -> bool {
        if self.is_clicked(position, events) {
            self.click_action(position, events);
            return true;
        }
        events.push(SceneEvent::PopLast);
        false
    }
}