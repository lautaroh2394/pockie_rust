use std::fmt::Debug;

use crate::{board::Board, character::Character, clickable::{Clickable, Position, Positionable}};

pub trait CharMenuOption {
    fn _execute(&self, c: Character, b: Board);
}

pub struct CharacterMenu {
    _options: Vec<Box<dyn CharMenuOption>>,
    position: Position,
}

impl CharacterMenu {
    pub fn new() -> CharacterMenu {
        CharacterMenu {
            _options: Vec::new(),
            position: Position::new(),
        }
    }
}

impl Debug for CharacterMenu {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Result::Ok(())
    }
}

impl Clone for CharacterMenu {
    fn clone(&self) -> Self {
        CharacterMenu {
            _options: Vec::new(),
            position: self.position.clone()
        }
    }
}

impl Positionable for CharacterMenu {
    fn get_pos(&self) -> &Position {
        &self.position
    }
}

impl Clickable for CharacterMenu {
    fn click_action(&mut self, _x: f32, _y: f32) {
        
    }
}