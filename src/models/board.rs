use macroquad::color::GRAY;
use macroquad::shapes::draw_rectangle;
use macroquad::window::{screen_height, screen_width};

use crate::enums::board_status::BoardStatus;
use crate::enums::event::SceneEvent;
use crate::traits::game_object::GameObject;
use crate::models::position::Position;

use crate::models::space::Space;

const DEFAULT_COLUMNS: i32 = 9;
const DEFAULT_ROWS: i32 = 5;

pub struct Board {
    pub _columns: i32,
    pub _rows: i32,
    pub map: Vec<Vec<Space>>,
    pub position: Position,
    pub status: BoardStatus,
}

impl Board {    
    pub fn toggle_status(board_status: &BoardStatus, new_space: &Space) -> Option<BoardStatus> {
        let new_space_clone = Some(new_space.clone());
        match board_status {
            BoardStatus::IDLE(_) => {
                Some(BoardStatus::SELECTING_MOVE(new_space_clone))
            },
            BoardStatus::SELECTING_MOVE(_) => {
                Some(BoardStatus::IDLE(new_space_clone))
            },
            _ => None            
        }
        
    }

    pub fn default() -> Board {
        Board::new(DEFAULT_COLUMNS, DEFAULT_ROWS)
    }
    pub fn new(total_columns: i32, total_rows: i32) -> Board {
        let mut map = Vec::new();

        let board_width = screen_width() * 0.8;
        let board_height = board_width / 2.2;
        let space_width = board_width / total_columns as f32;
        let space_height = board_height as f32 / total_rows as f32;

        let board_init_x = (screen_width() - board_width) / 2.0;
        let board_init_y = (screen_height() - board_height)/ 4.0;

        for row in 0..total_rows {
            let mut new_row = Vec::new();
            for column in 0..total_columns {
                new_row.push(Space::new(
                    (board_init_x + (column as f32 * space_width)) as f32,
                    (board_init_y + (row as f32 * space_height)) as f32,
                    space_width as f32,
                    space_height as f32,
                    column,
                    row
                ));
            }
            map.push(new_row);
        }

        Board {
            _columns: total_columns,
            _rows: total_rows,
            position: Position {
                x: board_init_x,
                y: board_init_y,
                w: board_width,
                h: board_height
            },
            map,
            status: BoardStatus::IDLE(None),
        }
    }
}

impl GameObject<SceneEvent> for Board {
    fn set_status(&mut self, status: BoardStatus) {
        self.status = status;
    }

    fn get_status(&self) -> &BoardStatus { &self.status }

    fn click_action(&mut self, position: &Position, events: &mut Vec<SceneEvent>) {
        for row in self.map.iter_mut() {
            let mut clicked = false;

            for space in row {
                if space.click(&position, events) {
                    clicked = true;
                    break; // No need to check the rest
                }
            }

            if clicked { break }
        }
    }
    
    fn get_pos(&self) -> &Position {
        &self.position
    }

    fn draw(&self){
        draw_rectangle(self.get_x(), self.get_y(),  self.get_width(), self.get_height(), GRAY);

        match &self.status {
            BoardStatus::SELECTING_MOVE(space_to_move) => {
                if let Some(e) = space_to_move {
                    for row in self.map.iter() {
                        for space in row {
                            if space.near(e) {
                                space.draw_selectable();
                            }
                            else {
                                space.draw();
                            }
                        }
                    }
                }
            },
            _ => {
                for row in self.map.iter() {
                    for space in row {
                        space.draw();
                    }
                }
            }
        }
    }
}
