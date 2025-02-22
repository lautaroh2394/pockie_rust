use crate::models::{fighter::Fighter, space::Space};


#[derive(Clone)]
pub enum BoardEvent {
    BoardIdle,
    BoardSelectVictim,
    BoardSelectMove(Space),
    BoardToggleIdleMove(Space),
}

#[derive(Clone)]
pub enum SceneEvent {
    CreateModal(Fighter),
    BoardEvent(BoardEvent),
    PopLast,
}