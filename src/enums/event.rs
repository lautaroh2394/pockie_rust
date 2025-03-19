use crate::models::{fighter::Fighter, space::Space};
use macroquad::ui::hash;


#[derive(Clone)]
pub enum BoardEvent {
    BoardIdle,
    BoardSelectVictim(Fighter),
    BoardSelectMove(Fighter),
    DropFighter(Fighter),
    SetFighter(Fighter),
    Attack(Fighter, Space),
}

#[derive(Clone)]
pub enum SceneEvent {
    CreateModal(CreateModalData),
    BoardEvent(BoardEventData),
    PopLast(PopLastData),
}

#[derive(Clone)]
pub struct CreateModalData {
    pub id: u64,
    pub fighter: Fighter,
}

#[derive(Clone)]
pub struct BoardEventData {
    pub id: u64,
    pub event: BoardEvent,
}

#[derive(Clone)]
pub struct PopLastData {
    pub id: u64,
}


impl SceneEvent {
    pub fn new_create_modal(fighter: Fighter) -> Self {
        let data = CreateModalData {
            id: hash!(),
            fighter,
        };
        SceneEvent::CreateModal(data)
    }

    pub fn new_board_event(board_event: BoardEvent) -> Self {
        let data = BoardEventData {
            id: hash!(),
            event: board_event
        };
        SceneEvent::BoardEvent(data)
    }

    pub fn new_pop_last() -> Self {
        SceneEvent::PopLast(PopLastData{ id: hash!()})
    }

    pub fn id(&self) -> u64 {
        match self {
            SceneEvent::CreateModal(data) => {
                data.id
            },
            SceneEvent::BoardEvent(data) => {
                data.id
            },
            SceneEvent::PopLast(data) => {
                data.id
            },
        }
    }
}