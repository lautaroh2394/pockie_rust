use macroquad::{prelude::*};
use crate::{character::Character, clickable::{Clickable, Position, Positionable}};

#[derive(Debug, Clone)]
pub struct Space {
    pub character: Option<Character>,
    pub position: Position,
    toggled: bool,
}

impl Positionable for Space { 
    fn get_pos(&self) -> &Position {
        &self.position
    }
}

impl Clickable for Space {
    fn click_action(&mut self, x: f32, y: f32){
        if let Some(c) = &mut self.character {
            c.click(x, y);
            return;
        } 
        self.toggled = !self.toggled;
    }
}

impl Space {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Space {
        Space {
            character: None,
            position: Position {x, y, w, h,},
            toggled: false,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.get_x(), self.get_y(), self.get_width(), self.get_height(), WHITE);
        let mut color = YELLOW;
        if self.toggled { color = GREEN} ;
        draw_rectangle(self.get_x() + 10.0, self.get_y() + 10.0, self.get_width() - 20.0, self.get_height() - 20.0, color);
        if let Some(c) = &self.character {
            c.draw();
        }
    }

    pub fn set_character(&mut self, mut c: Character){
        c.position = self.get_pos().clone(); // todo - improve
        self.character = Some(c);
    }
}

