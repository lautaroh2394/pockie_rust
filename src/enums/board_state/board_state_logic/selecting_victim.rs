use crate::{enums::{board_events::BoardEvent, event::SceneEvent}, global_events::push_global_event, models::{board::Board, fighter::Fighter, position::Position, space::Space}, traits::game_object::GameObject};

pub struct SelectingVictimLogic {}
impl SelectingVictimLogic {
    pub fn click_action(&self, map: &mut Vec<Vec<Space>>, position: &Position, fighter_option: Option<Fighter>) {
        let mut clicked = false;
        if let Some(fighter) = fighter_option {
            for row in map.iter_mut() {
                for space in row {
                    if space.is_clicked(&position) && !space.fighter.is_none() && fighter.can_attack_to(space) {
                        push_global_event(SceneEvent::board_event(BoardEvent::Attack(fighter.clone(), space.clone())));
                        clicked = true;
                        break; // No need to check the rest
                    }
                }
                if clicked { break }
            }
        }
        push_global_event(SceneEvent::board_event(BoardEvent::BoardIdle));
    }

    pub fn draw(&self, board: &Board, fighter_option: Option<Fighter>) {
        if let Some(e) = fighter_option {
                    
            for row in board.map.iter() {
                for space in row {
                    if e.can_attack_to(space) {
                        space.draw_attackable();
                    }
                    else {
                        space.draw();
                    }
                }
            }
        }
    }
}