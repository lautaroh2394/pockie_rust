use macroquad::{color::{GREEN, PURPLE, WHITE, YELLOW}, shapes::draw_rectangle};

use crate::{enums::event::Event, models::position::Position, traits::game_object::GameObject};

const SPACE_PAD: f32 = 10.0;

pub struct Space {
    pub position: Position,
    pub toggled: bool,
    pub x_index: i32,
    pub y_index: i32
}

impl Space {
    pub fn clone(&self) -> Self {
        Space {
            position: self.position.clone(),
            toggled: self.toggled,
            x_index: self.x_index,
            y_index: self.y_index,
        }
    }
    pub fn new(x: f32, y: f32, w: f32, h: f32, x_index: i32, y_index: i32) -> Space {
        Space {
            position: Position {x, y, w, h,},
            toggled: false,
            x_index,
            y_index,
        }
    }

    pub fn near(&self, space: &Space) -> bool {
        let horizontal = (space.x_index - self.x_index).abs();
        let vertical = (space.y_index - self.y_index).abs();
        horizontal + vertical <= 2
    }

    pub fn draw_selectable(&self) {
        draw_rectangle(
            self.get_x(), 
            self.get_y(), 
            self.get_width(),
            self.get_height(),
            WHITE
        );

        draw_rectangle(
            self.get_x() + SPACE_PAD,
            self.get_y() + SPACE_PAD,
            self.get_width() - SPACE_PAD * 2.0,
            self.get_height() - SPACE_PAD * 2.0, PURPLE);   
    }
}


impl GameObject for Space {
    // todo: events should be "scene_events"?
    fn click_action(&mut self, _: &Position, events: &mut Vec<Event>){
        self.toggled = !self.toggled;
        events.push(Event::BoardToggleIdleMove(self.clone()));
    }

    fn default_click_condition(&self, position: &Position, _: &mut Vec<Event>) -> bool {
        let overlaps_x = (self.get_x() + SPACE_PAD <= position.x) && (self.get_x() + self.get_width() - SPACE_PAD >= position.x);
        let overlaps_y = (self.get_y() + SPACE_PAD <= position.y) && (self.get_y() + self.get_height() - SPACE_PAD >= position.y);
        overlaps_x && overlaps_y
    }

    
    fn draw(&self) {
        draw_rectangle(
            self.get_x(), 
            self.get_y(), 
            self.get_width(),
            self.get_height(),
            WHITE
        );

        let mut color = YELLOW;
        if self.toggled { color = GREEN; } ;
        draw_rectangle(
            self.get_x() + SPACE_PAD,
            self.get_y() + SPACE_PAD,
            self.get_width() - SPACE_PAD * 2.0,
            self.get_height() - SPACE_PAD * 2.0, color);   
    }
    
    fn get_pos(&self) -> &Position {
        &self.position
    }
}