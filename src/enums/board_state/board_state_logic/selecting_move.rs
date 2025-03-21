use crate::{
    enums::{board_events::BoardEvent, event::SceneEvent}, 
    global_events::push_global_event, 
    models::{board::Board, fighter::Fighter, position::Position, space::Space}, traits::game_object::GameObject
};

pub struct SelectingMoveLogic {}
impl SelectingMoveLogic {
    pub fn click_action(&self, map: &mut Vec<Vec<Space>>, position: &Position, moving_fighter_option: Option<Fighter>) {
        let mut clicked = false;
        if let Some(fighter) = moving_fighter_option {
            for row in map.iter_mut() {
                for space in row {
                    if space.is_clicked(&position) && space.fighter.is_none() && fighter.can_move_to(space) {
                        push_global_event(SceneEvent::board_event(BoardEvent::DropFighter(fighter.clone())));
                        let mut f = fighter.clone();
                        f.update_map_position(&space);
                        push_global_event(SceneEvent::board_event(BoardEvent::SetFighter(f)));
                        push_global_event(SceneEvent::board_event(BoardEvent::BoardIdle));
                        clicked = true;
                        break; // No need to check the rest
                    }
                }
                if clicked { break }
            }
        }
    }

    pub fn draw(&self, board: &Board, fighter: Option<Fighter>) {
        if let Some(e) = fighter {
            let container_space = &board.map[e.y_index as usize][e.x_index as usize];
            for row in board.map.iter() {
                for space in row {
                    if space.near(&container_space) {
                        space.draw_selectable();
                    }
                    else {
                        space.draw();
                    }
                }
            }
        }
    }
}
