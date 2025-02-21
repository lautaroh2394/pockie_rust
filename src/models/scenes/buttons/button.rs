use crate::models::position::Position;

pub struct Button {
    position: Position,
    text: String,
}

impl Button {
    pub fn new(p: Position, t: String) -> Self {
        Button {
            position: p,
            text: t
        }
    }
}