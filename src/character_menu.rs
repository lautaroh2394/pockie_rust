use std::fmt::Debug;

use crate::{board::Board, character::Character, clickable::{Clickable, Position, Positionable}};

pub trait CharMenuOption {
    fn execute(&self, c: Character, b: Board);
}

pub struct CharacterMenu {
    options: Vec<Box<dyn CharMenuOption>>,
    position: Position,
}

impl CharacterMenu {
    pub fn new() -> CharacterMenu {
        CharacterMenu {
            options: Vec::new(),
            position: Position::new(),
        }
    }
}

impl Debug for CharacterMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Result::Ok(())
    }
}

impl Clone for CharacterMenu {
    fn clone(&self) -> Self {
        CharacterMenu {
            options: Vec::new(),
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
    fn click_action(&mut self, x: f32, y: f32) {
        
    }
}