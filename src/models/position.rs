use macroquad::window::{screen_height, screen_width};

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Position {
    pub fn clone(&self) -> Self {
        Position {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    pub fn default() -> Self {
        let x = 0.;
        let y = 0.;
        let h = screen_height();
        let w = screen_width();
        Position {x, y, w, h}
    }
}