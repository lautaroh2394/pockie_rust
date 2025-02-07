use std::{cell::RefCell, ops::Deref, rc::Rc};

use macroquad::{math::Vec2, ui::{hash, root_ui, Ui}, window::{screen_height, screen_width}};

use crate::{
    clickable::{Clickable, Position, Positionable}, drawable::Drawable, events::{Event, Modal}, scene_manager::SceneManager, screenable::Screenable
};

pub struct Scene<'a> {
    elements: Vec<Box<dyn Screenable + 'a>>,
    position: Position,
    is_modal: bool,
    events: Vec<Event>,
    manager: Rc<RefCell<SceneManager<'a>>>,
    callback: Option<&'a Modal>
}

impl<'a> Scene<'a> {
    pub fn new(p: Position,m: Rc<RefCell<SceneManager<'a>>>) -> Scene<'a> {
        Scene {
            elements: Vec::new(),
            position: p,
            is_modal: false,
            events: Vec::new(),
            manager: m,
            callback: None,
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

    pub fn new_modal(p: Position, m: Rc<RefCell<SceneManager<'a>>>, modal_info: &'a Modal) -> Scene<'a> {
        let mut s: Scene<'a> = Scene::new_full_screen(m);
        s.position = p;
        s.is_modal = true;
        s.callback = Some(modal_info);
        s
    }

    pub fn push<T: Screenable + 'a>(&mut self, element: T){
        self.elements.push(Box::new(element));
    }

    pub fn manage_events(&'a self, manager_ref: Rc<RefCell<SceneManager<'a>>>) {
        for event in self.events.iter(){
            match event {
                Event::CreateModal(modal_info) => {
                    manager_ref.clone().borrow_mut().push_scene(Scene::new_modal(modal_info.position.clone(), manager_ref.clone(), modal_info));
                },
                _ => ()
            };
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
        if self.is_modal {
            let modal_info = self.callback.unwrap();
            root_ui().window(
                hash!(),
                Vec2::new(100.0, 100.0),
                Vec2::new(200.0, 500.0),
                |ui| {
                    (&modal_info.f)(ui);
                }
            );
        }
        else {
            for element in self.elements.iter() {
                element.draw();
            }
        }
    }
}

impl<'a> Clickable for Scene<'a> {
    fn click_action(&mut self, x: f32, y: f32, _: &mut Vec<Event>) {
        for element in self.elements.iter_mut() {
            element.click(x, y, &mut self.events);
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