use crate::models::fighter::Fighter;

pub enum BoardState {
    Idle,
    SelectingMove(Option<Fighter>),
    SelectingVictim(Option<Fighter>),
}