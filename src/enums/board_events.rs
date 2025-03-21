use crate::models::{fighter::Fighter, space::Space};

#[derive(Clone)]
pub enum BoardEvent {
    BoardIdle,
    BoardSelectVictim(Fighter),
    BoardSelectMove(Fighter),
    DropFighter(Fighter),
    SetFighter(Fighter),
    Attack(Fighter, Space),
}