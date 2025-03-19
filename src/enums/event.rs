use crate::models::{fighter::Fighter, space::Space};


#[derive(Clone)]
pub enum BoardEvent {
    BoardIdle,
    BoardSelectVictim(Fighter),
    BoardSelectMove(Fighter),
    DropFighter(Fighter),
    SetFighter(Fighter),
    BoardToggleIdleMove(Space),
    CreateModalEvent(Fighter),
    Attack(Fighter, Space),
    UpdateFighter(Fighter),
}

#[derive(Clone)]
pub enum SceneEvent {
    CreateModal(Fighter),
    BoardEvent(BoardEvent),
    PopLast,
}