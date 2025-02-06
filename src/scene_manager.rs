use std::{cell::RefCell, rc::Rc};

use crate::{board::Board, character::Character, clickable::Clickable, drawable::Drawable, scene::Scene};

pub struct SceneManager<'a> {
    scenes: Vec<Scene<'a>>,
}

impl<'a> SceneManager<'a> {
    pub fn new_testing_scene(manager_ref: Rc<RefCell<SceneManager>>) {
        let mut board: Board = Board::default();
        let c = Character::new();
        board.set_character_to_coordinate(c, 4, 5);
        let mut scene = Scene::new_full_screen(manager_ref.clone());
        scene.push(board);
        manager_ref.clone().borrow_mut().push(scene);
    }

    pub fn new() -> SceneManager<'a>{
        SceneManager { scenes: Vec::new() }
    }

    pub fn _new_scene(self) {
        let ref_to_self = Rc::new(RefCell::new(self));
        let s = Scene::new_full_screen(ref_to_self.clone());
        ref_to_self.clone().borrow_mut().scenes.push(s);
    }

    pub fn push(&mut self, scene: Scene<'a>){
        self.scenes.push(scene);
    }

    pub fn manage_events(&self) {
        let ref_to_self: Rc<RefCell<&SceneManager>> = Rc::new(RefCell::new(&self));
        for scene in ref_to_self.clone().borrow().scenes.iter().rev() {
            let ref_to_scene: Rc<RefCell<& Scene<'a>>> = Rc::new(RefCell::new(scene));
            ref_to_scene.clone().borrow_mut().manage_events();
            if !ref_to_scene.clone().borrow().is_modal(){
                break;
            }
        }
    }
}

impl SceneManager<'_> {
    pub fn draw(&mut self) {
        self.scenes.last_mut().unwrap().draw();
    }

    pub fn click(&mut self, x: f32, y: f32) {
       self.scenes.last_mut().unwrap().click(x, y);
    }
}