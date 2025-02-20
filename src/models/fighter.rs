use macroquad::{color::PINK, shapes::draw_rectangle};

use crate::{
    enums::event::SceneEvent, 
    traits::game_object::GameObject,
    models::position::Position};

pub struct Fighter {
    position: Position,
    attack: i32,
    defense: i32,
    movement: i32,
    name: String,
}

impl Fighter {
    pub fn new(p: Position, name: String) -> Fighter {
        Fighter {
            position: p,
            attack: 5,
            defense: 3,
            movement: 2,
            name: name,
        }
    }

    pub fn clone(&self) -> Fighter {
        Fighter {
            position: self.position.clone(),
            attack: self.attack,
            defense: self.defense,
            movement: self.movement,
            name: self.name.to_string(),
        }
    }

    pub fn get_atk(&self) -> i32 { self.attack }
    pub fn get_def(&self) -> i32 { self.defense }
    pub fn get_movement(&self) -> i32 { self.movement }

}
impl GameObject<SceneEvent> for Fighter {
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