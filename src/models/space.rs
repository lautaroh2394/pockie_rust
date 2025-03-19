use macroquad::{color::{GREEN, ORANGE, PURPLE, WHITE, YELLOW}, shapes::draw_rectangle};

use crate::{
    enums::event::SceneEvent, 
    models::position::Position, 
    traits::game_object::GameObject
};

use super::fighter::Fighter;

pub const SPACE_PAD: f32 = 10.; // todo. move to other file? set as Space::SPACE_PAD ?

#[derive(Clone)]
pub struct Space {
    pub position: Position,
    pub toggled: bool,
    pub x_index: i32,
    pub y_index: i32,
    pub fighter: Option<Fighter>,
}

impl Space {
    pub fn clone(&self) -> Self {
        let mut f = None;
        if let Some(fig) = &self.fighter {
            f = Some(fig.clone());
        }
        Space {
            position: self.position.clone(),
            toggled: self.toggled,
            x_index: self.x_index,
            y_index: self.y_index,
            fighter: f,
        }
    }
    pub fn new(x: f32, y: f32, w: f32, h: f32, x_index: i32, y_index: i32) -> Space {
        Space {
            position: Position {x, y, w, h,},
            toggled: false,
            x_index,
            y_index,
            fighter: None
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
            self.get_width() - SPACE_PAD * 2.,
            self.get_height() - SPACE_PAD * 2., PURPLE
        );

        if let Some(f) = &self.fighter {
            f.draw();
        }
    }

    pub fn draw_attackable(&self) {
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
            self.get_width() - SPACE_PAD * 2.,
            self.get_height() - SPACE_PAD * 2., ORANGE
        );

        if let Some(f) = &self.fighter {
            f.draw();
        }
    }
    pub fn set_fighter(&mut self, fighter: Fighter) {
        println!("{}", fighter.hp);
        self.fighter = Some(fighter);
    }

    pub fn drop_fighter(&mut self) {
        self.fighter = None
    }
}


impl GameObject for Space {
    // todo: events should be "scene_events"?
    fn click_action(&mut self, position: &Position, events: &mut Vec<SceneEvent>){
        if let Some(f) = &mut self.fighter {
            f.click(position, events);
        }
    }

    fn default_click_condition(&self, position: &Position) -> bool {
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
            self.get_width() - SPACE_PAD * 2.,
            self.get_height() - SPACE_PAD * 2., color
        );   

        if let Some(f) = &self.fighter {
            f.draw();
        }
    }
    
    fn get_pos(&self) -> &Position {
        &self.position
    }
}