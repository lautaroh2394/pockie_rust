use std::{cell::RefCell, rc::Rc};

use crate::{board::Board, character::Character, clickable::{Clickable, Position}, drawable::Drawable, events::{Event, Modal}, scene::Scene, screenable::Screenable};

pub struct SceneManager<'a> {
    scenes: Vec<Scene<'a>>,
    events: Vec<Event>
}

impl<'a> SceneManager<'a> {
    pub fn new_testing_scene(manager_ref: Rc<RefCell<SceneManager>>) {
        let mut board: Board = Board::default();
        let c = Character::new();
        board.set_character_to_coordinate(c, 4, 5);
        let mut scene = Scene::new_full_screen(manager_ref.clone());
        scene.push(board);
        manager_ref.clone().borrow_mut().push_scene(scene);
    }

    pub fn new() -> SceneManager<'a>{
        SceneManager { scenes: Vec::new(), events: Vec::new() }
    }

    pub fn new_scene(manager_ref: Rc<RefCell<SceneManager>>){
        //let ref_to_self = Rc::new(RefCell::new(self));
        let s = Scene::new_full_screen(manager_ref.clone());
        manager_ref.clone().borrow_mut().scenes.push(s);
    }

    pub fn push_to_last_scene<T: Screenable + 'a>(&self, manager_ref: Rc<RefCell<SceneManager<'a>>>, elem: T){
        manager_ref.clone().borrow_mut().scenes.last_mut().unwrap().push(elem);
    }

    pub fn add_modal(mut self, position: Position, callback: &'a Modal){
        let manager_ref = Rc::new(RefCell::new(self));
        manager_ref.clone().borrow_mut().push_scene(Scene::new_modal(position, manager_ref, callback));
    }

    pub fn push_scene(&mut self, scene: Scene<'a>){
        self.scenes.push(scene);
    }

    pub fn pop_scene(&mut self){
        self.scenes.pop();
    }

    pub fn manage_events(&'a self, manager_ref: Rc<RefCell<SceneManager<'a>>>) {
        for scene in self.scenes.iter().rev() {
            let ref_to_scene: Rc<RefCell<& Scene<'a>>> = Rc::new(RefCell::new(scene));
            ref_to_scene.clone().borrow_mut().manage_events(manager_ref.clone());
        }
    }
}

impl SceneManager<'_> {
    pub fn draw(&mut self) {
        self.scenes.last_mut().unwrap().draw();
    }

    pub fn click(&mut self, x: f32, y: f32) {
       let scene = self.scenes.last_mut().unwrap();
       let is_clicked = scene.click(x, y, &mut self.events);
       if !is_clicked && scene.is_modal() { self.pop_scene() }
    }
}