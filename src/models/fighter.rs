use macroquad::{color::PINK, shapes::draw_rectangle};

use crate::{
    enums::event::SceneEvent, models::position::Position, traits::game_object::GameObject};

use super::space::Space;

#[derive(Clone)]
pub struct Fighter {
    position: Position,
    attack: i32,
    defense: i32,
    movement: i32,
    name: String,
    pub x_index: i32,
    pub y_index: i32,
}

impl Fighter {
    pub fn new(p: Position, name: String, space: &Space) -> Fighter {
        Fighter {
            position: p,
            attack: 5,
            defense: 3,
            movement: 2,
            name: name,
            x_index: space.x_index,
            y_index: space.y_index
        }
    }

    pub fn clone(&self) -> Fighter {
        Fighter {
            position: self.position.clone(),
            attack: self.attack,
            defense: self.defense,
            movement: self.movement,
            name: self.name.to_string(),
            x_index: self.x_index,
            y_index: self.y_index
        }
    }

    pub fn get_atk(&self) -> i32 { self.attack }
    pub fn get_def(&self) -> i32 { self.defense }
    pub fn get_movement(&self) -> i32 { self.movement }

    pub fn update_map_position(&mut self, space: &Space) {
        self.x_index = space.x_index;
        self.y_index = space.y_index;
        self.position = space.position.clone();
    }

    pub fn can_move_to(&self, space: &Space) -> bool {
        let horizontal = (space.x_index - self.x_index).abs();
        let vertical = (space.y_index - self.y_index).abs();
        horizontal + vertical <= 2
    }

}
impl GameObject for Fighter {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn draw(&self) {
        draw_rectangle(
            self.get_x() + (self.get_width() / 4.), 
            self.get_y() + self.get_height() / 4.,
            self.get_width() /2.,
            self.get_height() /2.,
            PINK
        )
    }

    fn get_pos(&self) -> &Position {
        &self.position
    }

    fn click_action(&mut self, _position: &Position, events: &mut Vec<SceneEvent>) {
        println!("fighter clicked");
        events.push(
            SceneEvent::CreateModal(self.clone())
            /*
            SceneEvent::BoardEvent(
                BoardEvent::BoardToggleIdleMove(self.clone())
            )
             */
        );
    }
}