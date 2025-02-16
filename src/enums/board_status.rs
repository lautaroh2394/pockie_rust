use crate::models::space::Space;

pub enum BoardStatus {
    IDLE(Option<Space>),
    SELECTING_MOVE(Option<Space>),
    SELECTING_VICTIM(Option<Space>),
}