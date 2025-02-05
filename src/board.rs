use macroquad::{prelude::*};
use crate::clickable::{Clickable, Position, Positionable};
use crate::drawable::Drawable;
use crate::scene::Screenable;
use crate::space::Space;
use crate::character::Character;

pub struct Board {
    pub columns: i32,
    pub rows: i32,
    pub map: Vec<Vec<Space>>,
    pub position: Position,
}

impl Positionable for Board {
    fn get_pos(&self) -> &Position {
        &self.position
    }
}

impl Clickable for Board {
    fn click_action(&mut self, x: f32, y: f32) {
        for row in self.map.iter_mut() {
            let mut clicked = false;

            for space in row {
                println!("Space clicked?");
                if space.click(x, y) {
                    println!("Space clicked");
                    clicked = true;
                    break; // No need to check the rest
                } else {println!("Space not clicked");}
            }

            if clicked { break }
        }
    }
}

impl Board {
    pub fn new() -> Board {
        let mut map = Vec::new();

        const TOTAL_COLUMNS: i32 = 9;
        const TOTAL_ROWS: i32 = 5;

        let board_width = screen_width() * 0.8;
        let board_height = board_width / 2.2;
        let space_width = (board_width / TOTAL_COLUMNS as f32);
        let space_height = (board_height as f32 / TOTAL_ROWS as f32);

        let board_init_x = (screen_width() - board_width) / 2.0;
        let board_init_y = (screen_height() - board_height)/ 4.0;

        println!("board_width {board_width}");
        println!("board_height {board_height}");
        println!("board_init_x {board_init_x}");
        println!("board_init_y {board_init_y}");
        println!("space_width {space_width}");
        println!("space_height {space_height}");
        

        for row in 0..TOTAL_ROWS {
            let mut new_row = Vec::new();
            for column in 0..TOTAL_COLUMNS {
                new_row.push(Space::new(
                    (board_init_x + (column as f32 * space_width)) as f32,
                    (board_init_y + (row as f32 * space_height)) as f32,
                    space_width as f32,
                    space_height as f32,
                ));
            }
            map.push(new_row);
        }

        Board {
            columns: TOTAL_COLUMNS,
            rows: TOTAL_ROWS,
            position: Position {
                x: board_init_x,
                y: board_init_y,
                w: board_width,
                h: board_height
            },
            map,
        }
    }

    pub fn set_character_to_coordinate(&mut self, c: Character, x: i32, y: i32) {
        let row = self.map.get_mut(x as usize).unwrap();
        let space = row.get_mut(y as usize).unwrap();

        space.set_character(c);
    }
}

impl Drawable for Board {
    fn draw(&self){
        draw_rectangle(self.get_x(), self.get_y(),  self.get_width(), self.get_height(), GRAY);
        for row in self.map.iter() {
            for space in row {
                space.draw();
            }
        }
    }
}

impl Screenable for Board {
    
}