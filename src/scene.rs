use std::{cell::RefCell, rc::Rc};

use macroquad::window::{screen_height, screen_width};

use crate::{
    clickable::{Clickable, Position, Positionable}, drawable::Drawable, events::Event, scene_manager::SceneManager, screenable::Screenable
};

pub struct Scene<'a> {
    elements: Vec<Box<dyn Screenable + 'a>>,
    position: Position,
    is_modal: bool,
    events: Vec<Event<'a>>,
    manager: Rc<RefCell<SceneManager<'a>>>,
}

impl<'a> Scene<'a> {
    pub fn new(p: Position,m: Rc<RefCell<SceneManager<'a>>>) -> Scene<'a> {
        Scene {
            elements: Vec::new(),
            position: p,
            is_modal: false,
            events: Vec::new(),
            manager: m,
        }
    }

    pub fn new_full_screen(m: Rc<RefCell<SceneManager<'a>>>) -> Scene<'a>{
        let x = 0.0;
        let y = 0.0;
        let h = screen_height();
        let w = screen_width();
        let p = Position {x, y, w, h};

        Scene::new(p, m)
    }

    pub fn _new_modal(p: Position, m: Rc<RefCell<SceneManager<'a>>>) -> Scene<'a> {
        let mut s = Scene::new_full_screen(m);
        s.position = p;
        s.is_modal = true;
        s
    }

    pub fn push<T: Screenable + 'a>(&mut self, element: T){
        self.elements.push(Box::new(element));
    }

    pub fn manage_events(&self) {
        for callback in self.events.iter(){
            callback(self.manager.clone());
        }
    }

    pub fn is_modal(&self) -> bool {
        self.is_modal
    }
}

impl<'a> Positionable for Scene<'a> {
    fn get_pos(&self) -> &Position {
        &self.position
    }
}

impl<'a> Drawable for Scene<'a> {
    fn draw(&self) {
        for element in self.elements.iter() {
            element.draw();
        }
    }
}

impl<'a> Clickable for Scene<'a> {
    fn click_action(&mut self, x: f32, y: f32) {
        for element in self.elements.iter_mut() {
            element.click(x, y);
        }
    }

    fn is_clicked(&self, x: f32, y: f32) -> bool {
        if self.is_modal {
            let mut some_clicked = false;
            for el in self.elements.iter() {
                if el.is_clicked(x, y){ some_clicked = true; break}
            }
            return some_clicked;
        }
        self.default_click_condition(x, y)
    }
}