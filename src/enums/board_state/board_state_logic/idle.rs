use crate::{models::{board::Board, position::Position, space::Space}, traits::game_object::GameObject};

pub struct IdleLogic {}
impl IdleLogic {
    pub fn click_action(&self, map: &mut Vec<Vec<Space>>, position: &Position) {
        let mut clicked = false;
        for row in map.iter_mut() {
            for space in row {
                if space.click(&position) {
                    clicked = true;
                    break; // No need to check the rest
                }
            }
            if clicked { break }
        }
    }

    pub fn draw(&self, board: &Board) {
        for row in board.map.iter() {
            for space in row {
                space.draw();
            }
        }
    }
}
