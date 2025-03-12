use crate::{enums::{board_status::BoardState, event::SceneEvent}, models::position::Position};

pub trait GameObject {
    fn draw(&self);
    fn click_action(&mut self, position: &Position, events: &mut Vec<SceneEvent>);

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
    
    fn click(&mut self, position: &Position, events: &mut Vec<SceneEvent>) -> bool {
        if self.is_clicked(position) {
            self.click_action(position, events);
            return true;
        }
        false
    }

    fn default_click_condition(&self, position: &Position) -> bool {
        let overlaps_x = (self.get_x() <= position.x) && (self.get_x() + self.get_width() >= position.x);
        let overlaps_y = (self.get_y() <= position.y) && (self.get_y() + self.get_height() >= position.y);
        overlaps_x && overlaps_y
    }
    

    fn is_clicked(&self, position: &Position) -> bool{ 
        self.default_click_condition(position)
    }

    fn get_name(&self) -> String { String::from("Nombre sin definir") }
    fn set_status(&mut self, _: BoardState){}
    fn get_status(&self) -> &BoardState { &BoardState::Idle }
    fn manage_events(&mut self, _events: &mut Vec<SceneEvent>) {}
}