use crate::events::Event;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Position {
    pub fn new()-> Position {
        Position {
            x: 0.0, y: 0.0, w: 0.0, h: 0.0
        }
    }
}


pub trait Positionable {
    fn get_pos(&self) -> &Position;

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

pub trait Clickable: Positionable {
    fn click_action(&mut self, x: f32, y: f32, events: &mut Vec<Event>);
    
    fn click(&mut self, x: f32, y: f32, events: &mut Vec<Event>) -> bool {
        if self.is_clicked(x, y) {
            self.click_action(x,y, events);
            return true;
        }
        return false;
    }

    fn default_click_condition(&self, x: f32, y: f32) -> bool {
        let overlaps_x = (self.get_x() <= x) && (self.get_x() + self.get_width() >= x);
        let overlaps_y = (self.get_y() <= y) && (self.get_y() + self.get_height() >= y);
        overlaps_x && overlaps_y
    }

    fn is_clicked(&self, x: f32, y: f32) -> bool {
        self.default_click_condition(x, y)
    }
}