use macroquad::color::GRAY;
use macroquad::shapes::draw_rectangle;
use macroquad::window::{screen_height, screen_width};

use crate::enums::board_status::BoardState;
use crate::enums::event::{BoardEvent, SceneEvent};
use crate::global_events::{global_events_iterate_mut, push_global_event, remove_global_event_by_id};
use crate::traits::game_object::GameObject;
use crate::models::position::Position;

use crate::models::space::Space;
use crate::GLOBAL_EVENTS;

const DEFAULT_COLUMNS: i32 = 9;
const DEFAULT_ROWS: i32 = 5;

pub struct Board {
    pub _columns: i32,
    pub _rows: i32,
    pub map: Vec<Vec<Space>>,
    pub position: Position,
    pub state: BoardState,
}

impl Board {    
    pub fn toggle_status(board_status: &BoardState, new_space: &Space) -> Option<BoardState> {
        let new_space_clone = Some(new_space.clone());
        match board_status {
            /*
            BoardStatus::IDLE(_) => {
                Some(BoardStatus::SELECTING_MOVE(new_space_clone))
            },
            */
            BoardState::SelectingMove(_) => {
                Some(BoardState::Idle)
            },
            _ => None            
        }
        
    }

    pub fn default() -> Board {
        Board::new(DEFAULT_COLUMNS, DEFAULT_ROWS)
    }

    pub fn new(total_columns: i32, total_rows: i32) -> Board {
        let mut map = Vec::new();

        let board_width = screen_width() * 0.8;
        let board_height = board_width / 2.2;
        let space_width = board_width / total_columns as f32;
        let space_height = board_height as f32 / total_rows as f32;

        let board_init_x = (screen_width() - board_width) / 2.;
        let board_init_y = (screen_height() - board_height)/ 4.;

        for row in 0..total_rows {
            let mut new_row = Vec::new();
            for column in 0..total_columns {
                new_row.push(Space::new(
                    (board_init_x + (column as f32 * space_width)) as f32,
                    (board_init_y + (row as f32 * space_height)) as f32,
                    space_width as f32,
                    space_height as f32,
                    column,
                    row
                ));
            }
            map.push(new_row);
        }

        Board {
            _columns: total_columns,
            _rows: total_rows,
            position: Position {
                x: board_init_x,
                y: board_init_y,
                w: board_width,
                h: board_height
            },
            map,
            state: BoardState::Idle,
        }
    }
}

impl GameObject for Board {
    fn set_status(&mut self, state: BoardState) {
        self.state = state;
    }

    fn get_status(&self) -> &BoardState { &self.state }

    fn click_action(&mut self, position: &Position) {
            let mut clicked = false;

            match &self.state {
                BoardState::Idle => {
                    for row in self.map.iter_mut() {
                    for space in row {
                        if space.click(&position) {
                            clicked = true;
                            break; // No need to check the rest
                        }
                    }
                    if clicked { break }
                }
                },
                BoardState::SelectingMove(fighter_option) => {
                    if let Some(fighter) = fighter_option {
                        for row in self.map.iter_mut() {
                        for space in row {
                            if space.is_clicked(&position) && space.fighter.is_none() && fighter.can_move_to(space) {
                                push_global_event(SceneEvent::new_board_event(BoardEvent::DropFighter(fighter.clone())));
                                let mut f = fighter.clone();
                                f.update_map_position(&space);
                                push_global_event(SceneEvent::new_board_event(BoardEvent::SetFighter(f)));
                                push_global_event(SceneEvent::new_board_event(BoardEvent::BoardIdle));
                                clicked = true;
                                break; // No need to check the rest
                            }
                        }
                        if clicked { break }
                        }
                    }
                },
                BoardState::SelectingVictim(fighter_option) => {
                    if let Some(fighter) = fighter_option {
                        for row in self.map.iter_mut() {
                        for space in row {
                            if space.is_clicked(&position) && !space.fighter.is_none() && fighter.can_attack_to(space) {
                                push_global_event(SceneEvent::new_board_event(BoardEvent::Attack(fighter.clone(), space.clone())));
                                clicked = true;
                                break; // No need to check the rest
                            }
                        }
                        if clicked { break }
                    }
                    }
                    push_global_event(SceneEvent::new_board_event(BoardEvent::BoardIdle));
                }
                _ => ()
            };
    }
    
    fn get_pos(&self) -> &Position {
        &self.position
    }

    fn manage_events(&mut self) {
        global_events_iterate_mut(|event| {
            let mut exec = true;
            match event {
                SceneEvent::BoardEvent(board_event) => {
                    match &board_event.event {
                        BoardEvent::BoardSelectMove(fighter) => {
                            println!("board select move");
                            self.set_status(BoardState::SelectingMove(Some(fighter.clone())));
                            push_global_event(SceneEvent::new_pop_last());
                        },
                        BoardEvent::BoardIdle => {
                            println!("board SET IDLE");
                            self.set_status(BoardState::Idle);
                        },
                        BoardEvent::DropFighter(fighter) => {
                            let fighter_current_space = &mut self.map[fighter.y_index as usize][fighter.x_index as usize]; 
                            fighter_current_space.drop_fighter();
                        },
                        BoardEvent::SetFighter(fighter) => {
                            let fighter_current_space = &mut self.map[fighter.y_index as usize][fighter.x_index as usize]; 
                            fighter_current_space.set_fighter(fighter.clone());
                        },
                        BoardEvent::BoardSelectVictim(fighter) => {
                            println!("board select victim");
                            self.set_status(BoardState::SelectingVictim(Some(fighter.clone())));
                            push_global_event(SceneEvent::new_pop_last());
                        },
                        BoardEvent::Attack(attacker, space ) => {
                            if let Some(fighter) = &space.fighter {
                                let mut victim = fighter.clone();
                                attacker.attack(&mut victim);
                                let fighter_current_space = &mut self.map[victim.y_index as usize][victim.x_index as usize]; 
                                
                                if !victim.is_dead() {
                                    let clone = victim.clone();
                                    fighter_current_space.set_fighter(clone);
                                }
                                else {
                                    fighter_current_space.drop_fighter();
                                }
                            }
                        },
                        _ => { exec = false;}
                    }
                    if exec { remove_global_event_by_id(event.id());}
                },
                _ => ()
            }
        });
    }

    fn draw(&self){
        draw_rectangle(self.get_x(), self.get_y(),  self.get_width(), self.get_height(), GRAY);

        match &self.state {
            BoardState::SelectingMove(fighter) => {
                if let Some(e) = fighter {
                    let container_space = &self.map[e.y_index as usize][e.x_index as usize];
                    for row in self.map.iter() {
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
            },
            BoardState::SelectingVictim(fighter_option) => {
                if let Some(e) = fighter_option {
                    
                    for row in self.map.iter() {
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
            },
            _ => {
                for row in self.map.iter() {
                    for space in row {
                        space.draw();
                    }
                }
            }
        }
    }
}
