use macroquad::{color::{BLACK, WHITE}, shapes::draw_rectangle, text::draw_text};

use crate::{enums::event::SceneEvent, models::position::Position, traits::game_object::GameObject};

pub struct Button {
    position: Position,
    text: String,
    action: Box<dyn ButtonAction>
    
}

pub trait ButtonAction {
    fn execute(&self);
}

impl Button {
    pub fn new(p: Position, t: String, action: Box<dyn ButtonAction>) -> Self {
        Button {
            position: p,
            text: t,
            action
        }
    }
}

impl GameObject for Button {
    fn draw(&self) {
        draw_rectangle(
            self.get_x(), 
            self.get_y(),  
            self.get_width(), 
            self.get_height(),
            WHITE
        );
        draw_text(&self.text, self.get_x() + 10., self.get_y() + 20.,25., BLACK);
    }

    fn click_action(&mut self, _position: &Position) {
        self.action.execute()
    }

    fn get_pos(&self) -> &Position {
        &self.position
    }
}