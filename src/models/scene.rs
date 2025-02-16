
use macroquad::window::{screen_height, screen_width};
use crate::{
    enums::{
        event::Event, 
        board_status::BoardStatus,
    },
    traits::game_object::GameObject,
    models::{
        position::Position,
        board::Board,
    },
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

    pub fn manage_events(&mut self, events: &mut Vec<Event>){
        for event in self.events.iter() {
            match event {
                Event::BoardToggleIdleMove(space) => {
                    let status = self.elements[0].get_status();
                    let status = Board::toggle_status(status, space);
                    if let Some(s) = status {
                        self.elements[0].set_status(s);
                    }
                },
                Event::BoardSelectMove(space) => {
                    self.elements[0].set_status(BoardStatus::SELECTING_MOVE(Some(space.clone())));
                },
                _ => {}
            }
        }
        self.events = Vec::new();
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