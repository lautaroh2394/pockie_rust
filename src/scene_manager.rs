use std::{cell::RefCell, rc::Rc};

use crate::{board::Board, character::Character, clickable::Clickable, drawable::Drawable, scene::Scene};

pub struct SceneManager<'a> {
    scenes: Vec<Scene<'a>>,
    current: Option<&'a Scene<'a>>,
}

impl<'a> SceneManager<'a> {
    pub fn new_testing_scene(&mut self) {
        let mut board: Board = Board::new();
        let c = Character::new();
        board.set_character_to_coordinate(c, 4, 5);
        let mut scene = Scene::new_full_screen();
        scene.push(board);
        //self.push(scene);
    }
    pub fn new() -> SceneManager<'a>{
        SceneManager { scenes: Vec::new(), current: None }
    }

    pub fn new_scene(&'a mut self) -> Scene<'a> {
        let ref_to_self = Rc::new(RefCell::new(self));
        let s = Scene::new_full_screen(ref_to_self.clone());
        ref_to_self.clone().borrow_mut().scenes.push(s);
        s
    }

    pub fn push(&mut self, scene: Scene<'a>){
        self.scenes.push(scene);
    }

    fn set_current(&mut self, scene: &'a Scene){
        //self.current = Some(&scene);
    }

    pub fn get_current(&mut self) -> &mut Scene<'a>{
        self.scenes.last_mut().unwrap()
    }

    pub fn manage_events(self) {
        //let ref_to_self = Rc::new(RefCell::new(self));
        /*
        self.scenes.last().unwrap().manage_events(self);
        let b = ref_to_self.clone();
        let br = b.borrow();
        let v = br.scenes.split_last().unwrap().0;
        v.manage_events(self);
        */
        for scene in self.scenes.iter().rev() {
            //let ref_to_scene: Rc<RefCell<& Scene<'_>>> = Rc::new(RefCell::new(scene));
            //ref_to_scene.clone().borrow_mut().manage_events(ref_to_self.clone());
            scene.manage_events();
            if (!scene.is_modal()){
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
       //self.scenes.last_mut().unwrap().click(x, y);
    }
}