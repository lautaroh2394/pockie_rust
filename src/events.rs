use std::{cell::RefCell, rc::Rc};

use macroquad::ui::{Id, Ui};

use crate::{clickable::Position, scene_manager::SceneManager};

//pub type Event<'a> = Box<dyn Fn(Rc<RefCell<SceneManager<'a>>>)>;


pub struct Modal {
    pub id: Id,
    pub position: Position,
    pub f: Box<dyn Fn(&mut Ui)>,
}
/*
pub struct EventBus {
    events: Vec<Event>,
}

//type Event = Option<Box<dyn Fn(&SceneManager)>>;

impl EventBus {
    fn push(&mut self, event: Event){
        self.events.push(event);
    }

    fn execute_events(&mut self, manager: &SceneManager){
        for event in self.events.iter() {
            if let Some(handler) = event {
                handler(manager);
            };
        }
        self.events = Vec::new();
    }
}
*/