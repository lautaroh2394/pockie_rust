use macroquad::{color::{GREEN, PURPLE, WHITE, YELLOW}, shapes::draw_rectangle};

use crate::{enums::event::Event, models::position::Position, traits::game_object::GameObject};

pub struct Space {
    pub position: Position,
    pub toggled: bool,
    pub x_index: i32,
    pub y_index: i32
}

impl Space {
    pub fn new(x: f32, y: f32, w: f32, h: f32, x_index: i32, y_index: i32) -> Space {
        Space {
            position: Position {x, y, w, h,},
            toggled: false,
            x_index,
            y_index,
        }
    }

    pub fn near(&self, space: &Space) -> bool {
        let horizontally_near = (space.x_index - self.x_index).abs() <= 2;
        let verticallly_near = (space.y_index - self.y_index).abs() <= 2;
        /*
        println!("
space.x_index => {},
self.x_index => {},
(space.x_index - self.x_index).abs() <= 2 => {},
horizontally_near = {}, 
space.y_index => {},
self.y_index => {},
(space.y_index - self.y_index).abs() <= 2 => {},
vertically_near = {}", 
        space.x_index, 
        self.x_index, 
        (space.x_index - self.x_index).abs(), 
        horizontally_near,
        space.y_index, 
        self.y_index, 
        (space.y_index - self.y_index).abs(), 
        verticallly_near
        );
         */
        horizontally_near && verticallly_near
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
            self.get_x() + 10.0,
            self.get_y() + 10.0,
            self.get_width() - 20.0,
            self.get_height() - 20.0, PURPLE);   
    }
}


impl GameObject for Space {
    // todo: events should be "scene_events"
    fn click_action(&mut self, _: &Position, events: &mut Vec<Event>){
        self.toggled = !self.toggled;
        events.push(Event::BoardSelectMove(Space {
            position: Position {
                x: self.get_x(),
                y: self.get_y(),
                w: self.get_width(),
                h: self.get_height()
            },
            toggled: false,
            x_index: self.x_index,
            y_index: self.y_index
        }));
    }

    fn default_click_condition(&self, position: &Position, _: &mut Vec<Event>) -> bool {
        let overlaps_x = (self.get_x() + 10.0 <= position.x) && (self.get_x() + self.get_width() - 10.0 >= position.x);
        let overlaps_y = (self.get_y() + 10.0 <= position.y) && (self.get_y() + self.get_height() - 10.0 >= position.y);
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
            self.get_x() + 10.0,
            self.get_y() + 10.0,
            self.get_width() - 20.0,
            self.get_height() - 20.0, color);   
    }
    
    fn get_pos(&self) -> &Position {
        &self.position
    }

    fn get_x(&self) -> f32 {
        self.get_pos().x
    }

    fn get_y(&self) -> f32 {
        self.get_pos().y
    }

    fn get_width(&self) -> f32 {
        self.get_pos().w
    }

    fn get_height(&self) -> f32 {
        self.get_pos().h
    }
}