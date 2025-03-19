use crate::global_events::{global_events_iterate_mut, push_global_event, remove_global_event_by_id, remove_global_events};
use crate::models::{
    scenes::scene::Scene,
    board::Board,
    position::Position,
    
};
use crate::enums::event::SceneEvent;
use crate::traits::game_object::GameObject;

use super::fighter::Fighter;
use super::scenes::modal::Modal;

pub struct SceneManager {
    scenes: Vec<Scene>,
}

impl SceneManager {
    pub fn new_testing_scene(&mut self){
        let mut scene: Scene = Scene::default();
        let mut board: Board = Board::default();
        let s = &mut board.map[0][3];
        s.fighter = Some(Fighter::new(s.position.clone(), "prueba".to_string(), s));

        let s = &mut board.map[4][1];
        s.fighter = Some(Fighter::new(s.position.clone(), "prueba".to_string(), s));
        scene.push(Box::new(board));
        self.push_scene(scene);
    }

    pub fn new() -> SceneManager{
        SceneManager { scenes: Vec::new()}
    }

    pub fn push_scene(&mut self, scene: Scene){
        self.scenes.push(scene);
    }

    pub fn manage_events(&mut self) {
        for scene in self.scenes.iter_mut().rev() {
            scene.manage_events();
        }
        
        global_events_iterate_mut(|event| {
            let mut exec = true;
            match event {
                SceneEvent::CreateModal(data) => {
                    self.scenes.push(Modal::new(data.fighter.clone()));
                },
                SceneEvent::PopLast(data) => {
                    self.scenes.pop();
                },
                _ => {
                    exec = false;
                }
            }
            if exec { remove_global_event_by_id((event.id()));}
        });

        remove_global_events();
    }
}

impl SceneManager {
    pub fn draw(&self) {
        let scene = self.scenes.last().unwrap();
        if scene.is_modal() { 
            self.scenes[self.scenes.len() - 2].draw();
        }
        scene.draw();
    }

    pub fn click(&mut self, x: f32, y: f32) {
       let scene = self.scenes.last_mut().unwrap();
       let position = Position { x, y, w:0., h:0.};
       scene.click(&position);
    }

    pub fn pop_scene(&mut self) {
        self.scenes.pop();
    }

    pub fn empty(&self) -> bool {
        self.scenes.len() == 0
    }
}