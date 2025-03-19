use crate::models::{fighter::Fighter, space::Space};

pub enum BoardState {
    Idle,
    SelectingMove(Option<Fighter>),
    SelectingVictim(Option<Fighter>),
}