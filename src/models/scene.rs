
use macroquad::window::{screen_height, screen_width};

use crate::{
    enums::event::Event, 
    traits::game_object::GameObject,
    models::position::Position,
};

pub struct Scene {
    pub elements: Vec<Box<dyn GameObject>>,
    pub events: Vec<Event>,
    pub position: Position
}

impl Scene {
    pub fn new() -> Scene {
        let x = 0.0;
        let y = 0.0;
        let h = screen_height();
        let w = screen_width();
        let p = Position {x, y, w, h};
        Scene {
            elements: Vec::new(),
            events: Vec::new(),
            position: p,
        }
    }
    
    pub fn push(&mut self, element: Box<dyn GameObject>){
        self.elements.push(element);
    }
}

impl GameObject for Scene {
    fn draw(&self) {
        for element in self.elements.iter() {
            element.draw();
        }
    }

    fn click_action(&mut self, position: &Position, _: &mut Vec<Event>) {
        for element in self.elements.iter_mut() {
            element.click(position, &mut self.events);
        }
    }

    fn get_pos(&self) -> &Position {
        &self.position
    }
}