use macroquad::color::GRAY;
use macroquad::shapes::draw_rectangle;
use macroquad::window::{screen_height, screen_width};

use crate::enums::board_status::BoardState;
use crate::enums::event::{BoardEvent, SceneEvent};
use crate::traits::game_object::GameObject;
use crate::models::position::Position;

use crate::models::space::Space;

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

    fn click_action(&mut self, position: &Position, events: &mut Vec<SceneEvent>) {
        for row in self.map.iter_mut() {
            let mut clicked = false;

            /* Click action should be according to state */
            match &self.state {
                BoardState::Idle => {
                    for space in row {
                        if space.click(&position, events) {
                            clicked = true;
                            break; // No need to check the rest
                        }
                    }
                },
                BoardState::SelectingMove(fighter_option) => {
                    if let Some(fighter) = fighter_option {
                        for space in row {
                            if space.is_clicked(&position) && space.fighter.is_none() && fighter.can_move_to(space) {
                                events.push(SceneEvent::BoardEvent(BoardEvent::DropFighter(fighter.clone())));
                                let mut f = fighter.clone();
                                f.update_map_position(&space);
                                events.push(SceneEvent::BoardEvent(BoardEvent::SetFighter(f)));
                                events.push(SceneEvent::BoardEvent(BoardEvent::BoardIdle));
                                clicked = true;
                                break; // No need to check the rest
                            }
                        }
                    }
                },
                _ => ()
            }

            if clicked { break }
        }
    }
    
    fn get_pos(&self) -> &Position {
        &self.position
    }

    fn manage_events(&mut self, events: &mut Vec<SceneEvent>) {
        let mut board_events = Vec::new();
        let mut new_events = Vec::<SceneEvent>::new();

        events.iter().for_each(|ev| {
            match ev {
                SceneEvent::BoardEvent(board_event) => {
                    board_events.push(board_event.clone());
                },
                ev => {
                    new_events.push(ev.clone());
                }
            }
        });
        
        events.clear();
        for ev in new_events.iter_mut() {
            events.push(ev.clone());
        }

        board_events.iter().for_each(|board_event| {
            match board_event {
                BoardEvent::BoardToggleIdleMove(space) => {
                    let status = self.get_status();
                    let status = Board::toggle_status(status, &space);
                    if let Some(s) = status {
                        self.set_status(s);
                    }
                },
                BoardEvent::BoardSelectMove(fighter) => {
                    println!("board select move");
                    self.set_status(BoardState::SelectingMove(Some(fighter.clone())));
                    events.push(SceneEvent::PopLast);
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
