pub struct Position {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Position {
    pub fn clone(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }
}