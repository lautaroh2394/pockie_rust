use macroquad::{color::GRAY, shapes::draw_rectangle};

use crate::{enums::event::SceneEvent, models::{fighter::Fighter, position::Position}, traits::game_object::GameObject};

use super::buttons::button::Button;

pub struct CharacterMenu {
    position: Position,
    options: Vec<Button>,

}

impl CharacterMenu {
    pub fn new_for_fighter(fighter: &Fighter) -> Self {
        let mut pos = fighter.get_pos().clone();
        pos.x += 50.;
        pos.y += 50.;

        let mut buttons: Vec<Button> = Vec::new();
        buttons.push(Button::new(pos.clone(), String::from("attack")));

        CharacterMenu {
            position: pos,
            options: buttons,
        }
    }
}

impl GameObject<SceneEvent> for CharacterMenu {
    fn draw(&self){
        draw_rectangle(
            self.get_x() + 50., 
            self.get_y() + 50.,  
            self.get_width(), 
            self.get_height(),
             GRAY
        );

        for button in self.options.iter() {
            button.draw();
        }
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