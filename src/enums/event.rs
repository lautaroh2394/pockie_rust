use crate::models::{fighter::Fighter, space::Space};


pub enum BoardEvent {
    BoardIdle,
    BoardSelectVictim,
    BoardSelectMove(Space),
    BoardToggleIdleMove(Space),
}

pub enum SceneEvent {
    CreateModal(Fighter),
    BoardEvent(BoardEvent),
}