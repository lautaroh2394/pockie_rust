use macroquad::{color::GRAY, shapes::draw_rectangle};

use crate::{models::{board::Board, fighter::Fighter, position::Position, space::Space}, traits::game_object::GameObject};

use super::board_state_logic::{idle::IdleLogic, selecting_move::SelectingMoveLogic, selecting_victim::SelectingVictimLogic};

pub enum BoardState {
    Idle(IdleLogic),
    SelectingMove(Option<Fighter>, SelectingMoveLogic),
    SelectingVictim(Option<Fighter>, SelectingVictimLogic),
}

impl BoardState {
    pub fn idle() -> BoardState {
        BoardState::Idle(IdleLogic {})
    }

    pub fn selecting_move(fighter_option: Option<Fighter>) -> BoardState {
        BoardState::SelectingMove(fighter_option, SelectingMoveLogic {})
    }

    pub fn selecting_victim(fighter_option: Option<Fighter>) -> BoardState {
        BoardState::SelectingVictim(fighter_option, SelectingVictimLogic {})
    }

    pub fn click_action(&self, map: &mut Vec<Vec<Space>>, position: &Position) {
        match self {
            BoardState::Idle(logic) => {
                logic.click_action(map, position);
            },
            BoardState::SelectingMove(fighter_option, logic, ) => {
                // todo - horrible cloning. stop cloning fighters everywhere
                let mut fighter = None;
                if let Some(f) = fighter_option {
                    fighter = Some(f.clone())
                }
                logic.click_action(map, position, fighter);
            },
            BoardState::SelectingVictim(fighter_option, logic, ) => {
                // todo - horrible cloning. stop cloning fighters everywhere
                let mut fighter = None;
                if let Some(f) = fighter_option {
                    fighter = Some(f.clone())
                }
                logic.click_action(map, position, fighter);
            }
        }
    }

    pub fn draw(&self, board: &Board) {
        draw_rectangle(board.get_x(),board.get_y(), board.get_width(), board.get_height(), GRAY);
        match self {
            BoardState::SelectingMove(fighter_option, logic) => {
                let mut f_option = None;
                if let Some(f) = fighter_option { f_option = Some(f.clone())}
                logic.draw(board, f_option);
            },
            BoardState::SelectingVictim(fighter_option, logic) => {
                let mut f_option = None;
                if let Some(f) = fighter_option { f_option = Some(f.clone())}
                logic.draw(board, f_option);
            },
            BoardState::Idle(logic) => {
                logic.draw(board);
            }
        }
    }
}