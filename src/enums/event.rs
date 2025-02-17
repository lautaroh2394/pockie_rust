use crate::models::space::Space;


pub enum BoardEvent {
    BoardIdle,
    BoardSelectVictim,
    BoardSelectMove(Space),
    BoardToggleIdleMove(Space),
}

pub enum SceneEvent {
    CreateModal,
    BoardEvent(BoardEvent),
}