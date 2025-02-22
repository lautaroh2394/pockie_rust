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

        let mut m = CharacterMenu {
            position: pos,
            options: Vec::new(),
        };

        m.add_options(vec![
            String::from("Attack"),
        ]);
        m
    }

    pub fn add_options(&mut self, options_config: Vec<String>) {
        let mut option_height = ((self.get_height() - (10. * 2.)) / options_config.len() as f32) % 30.;
        let option_width = (self.get_height() - (5. * 2.));
        let x = self.get_x() + 10.;

        for (i, option_title) in options_config.iter().enumerate() {
            self.options.push(Button::new(Position {
                x,
                w: option_width,
                h: option_height,
                y: self.get_y() + 10. + (5. * i as f32)
            }, String::from(option_title)))
        }


    } 
}

impl GameObject<SceneEvent> for CharacterMenu {
    fn draw(&self){
        draw_rectangle(
            self.get_x(), 
            self.get_y(),  
            self.get_width(), 
            self.get_height(),
             GRAY
        );

        for button in self.options.iter() {
            button.draw();
        }
    }

    fn click_action(&mut self, position: &Position, events: &mut Vec<SceneEvent>){
        for option in self.options.iter_mut() {
            option.click(position, events);
        }
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