use crate::{enums::{board_status::BoardStatus, event::Event}, models::position::Position};

pub trait GameObject {
    fn draw(&self);
    fn click_action(&mut self, position: &Position, events: &mut Vec<Event>);
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
    
    fn click(&mut self, position: &Position, events: &mut Vec<Event>) -> bool {
        if self.is_clicked(position, events) {
            self.click_action(position, events);
            return true;
        }
        false
    }

    fn default_click_condition(&self, position: &Position, _: &mut Vec<Event>) -> bool {
        let overlaps_x = (self.get_x() <= position.x) && (self.get_x() + self.get_width() >= position.x);
        let overlaps_y = (self.get_y() <= position.y) && (self.get_y() + self.get_height() >= position.y);
        overlaps_x && overlaps_y
    }
    

    fn is_clicked(&self, position: &Position, events: &mut Vec<Event>) -> bool{ 
        self.default_click_condition(position, events)
    }

    fn set_status(&mut self, _: BoardStatus){}
    fn get_status(&self) -> &BoardStatus { &BoardStatus::IDLE }
}