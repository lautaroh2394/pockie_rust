use crate::models::space::Space;

pub enum BoardStatus {
    IDLE,
    SELECTING_MOVE(Space),
    SELECTING_VICTIM,
}