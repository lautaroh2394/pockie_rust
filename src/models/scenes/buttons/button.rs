use macroquad::{color::{BLACK, WHITE}, shapes::draw_rectangle, text::draw_text};

use crate::{enums::event::SceneEvent, models::position::Position, traits::game_object::GameObject};

pub struct Button {
    position: Position,
    text: String,
}

impl Button {
    pub fn new(p: Position, t: String) -> Self {
        Button {
            position: p,
            text: t
        }
    }
}

impl GameObject<SceneEvent> for Button {
    fn draw(&self) {
        draw_rectangle(
            self.get_x(), 
            self.get_y(),  
            self.get_width(), 
            self.get_height(),
            WHITE
        );

        draw_text(&self.text, self.get_x() + 10., self.get_y() + 10., 10., BLACK);
    }

    fn click_action(&mut self, _position: &Position, _events: &mut Vec<SceneEvent>) {
        //todo
        println!("button clicked");
    }

    fn get_pos(&self) -> &Position {
        &self.position
    }
}