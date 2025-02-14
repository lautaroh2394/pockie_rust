use crate::enums::board_status::BoardStatus;
use crate::models::{
    scene::Scene,
    board::Board,
};
use crate::enums::event::Event;
use crate::traits::game_object::GameObject;

use super::position::Position;
use super::space::Space;


pub struct SceneManager {
    scenes: Vec<Scene>,
    events: Vec<Event>
}

impl SceneManager {
    pub fn new_testing_scene(&mut self){
        let mut scene: Scene = Scene::new();
        let board: Board = Board::default();
        scene.push(Box::new(board));
        self.push_scene(scene);
        
    }

    pub fn new() -> SceneManager{
        SceneManager { scenes: Vec::new(), events: Vec::new() }
    }

    pub fn push_scene(&mut self, scene: Scene){
        self.scenes.push(scene);
    }

    pub fn manage_events(&mut self) {
        for scene in self.scenes.iter_mut().rev() {
            for event in scene.events.iter() {
                match event {
                    Event::BoardSelectMove(space) => {
                        scene.elements[0].set_status(BoardStatus::SELECTING_MOVE(Space {
                            position: Position {
                                x: space.get_x(), y: space.get_y(), h: space.get_height(), w: space.get_height()
                            },
                            toggled: false,
                            x_index: space.x_index,
                            y_index: space.y_index
                        }));
                    },
                    _ => {}
                }
            }
        }
    }
}

impl SceneManager {
    pub fn draw(&self) {
        let scene = self.scenes.last().unwrap();
        scene.draw();
    }

    pub fn click(&mut self, x: f32, y: f32) {
       let scene = self.scenes.last_mut().unwrap();
       let position = Position { x, y, w:0.0, h:0.0};
       scene.click(&position, &mut self.events);
    }
}