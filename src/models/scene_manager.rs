use crate::models::{
    scene::Scene,
    board::Board,
    position::Position,
    
};
use crate::enums::event::SceneEvent;
use crate::traits::game_object::GameObject;

use super::fighter::Fighter;

pub struct SceneManager {
    scenes: Vec<Scene>,
    events: Vec<SceneEvent>
}

impl SceneManager {
    pub fn new_testing_scene(&mut self){
        let mut scene: Scene = Scene::new();
        let mut board: Board = Board::default();
        let s = &mut board.map[0][3];
        s.fighter = Some(Fighter::new(s.position.clone(), "prueba".to_string()));

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
            scene.manage_events(&mut self.events);
        }
        
        for event in self.events.iter() {
            match event {
                SceneEvent::CreateModal(f) => {
                    self.scenes.push(Scene::new_modal(f.clone()));
                },
                _ => ()
            }
        }
        self.events = Vec::new();
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
       let position = Position { x, y, w:0.0, h:0.0};
       scene.click(&position, &mut self.events);
    }

    pub fn pop_scene(&mut self) {
        self.scenes.pop();
    }

    pub fn empty(&self) -> bool {
        self.scenes.len() == 0
    }
}