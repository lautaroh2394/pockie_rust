
use crate::{
    enums::{
        board_status::BoardStatus, 
        event::{
            BoardEvent, SceneEvent
        }
    }, models::{
        board::Board, 
        position::Position, 
    }, traits::{
        game_object::GameObject, 
        render_logic::RenderLogic,
    },
    render_logic::default_scene_render_logic::DefaultSceneRenderLogic, 
};

use super::scene_creation_data::SceneCreationData;

pub struct Scene {
    pub elements: Vec<Box<dyn GameObject<SceneEvent>>>,
    pub events: Vec<SceneEvent>,
    pub position: Position,
    renderer: Box<dyn RenderLogic<Scene>>,
}

impl Scene {
    pub fn default() -> Self {
        Scene::new_from_data(SceneCreationData::default()) 
    }

    pub fn new_from_data(data: SceneCreationData) -> Self {
        let elements = if let Some(elements) = data.elements { elements } else { Vec::new() };
        let events = if let Some(events) = data.events { events } else { Vec::new() };
        let position = if let Some(position) = data.position { position } else { Position::default() };
        let renderer = if let Some(renderer) = data.renderer { renderer } else { Box::new(DefaultSceneRenderLogic {}) };

        Scene {
            elements,
            events,
            position,
            renderer,
        }
    }
    
    pub fn push(&mut self, element: Box<dyn GameObject<SceneEvent>>){
        self.elements.push(element);
    }

    /*
    pub fn manage_events(&mut self, events: &mut Vec<SceneEvent>){
        for event in self.events.iter() {
            match event {
                SceneEvent::BoardEvent(BoardEvent::BoardToggleIdleMove(space)) => {
                    let status = self.elements[0].get_status();
                    let status = Board::toggle_status(status, &space);
                    if let Some(s) = status {
                        self.elements[0].set_status(s);
                    }
                },
                SceneEvent::BoardEvent(BoardEvent::BoardSelectMove(space)) => {
                    self.elements[0].set_status(BoardStatus::SELECTING_MOVE(Some(space.clone())));
                },
                SceneEvent::CreateModal(f) => {
                    events.push(SceneEvent::CreateModal(f.clone()));
                },
                SceneEvent::PopLast => {
                    events.push(SceneEvent::PopLast);
                }
                _ => {}
            }
        }
        self.events = Vec::new();
    }
    */

    pub fn is_modal(&self) -> bool { self.renderer.is_modal() }
}

impl GameObject<SceneEvent> for Scene {
    fn draw(&self) {
        self.renderer.render(self);
    }

    fn click_action(&mut self, position: &Position, _: &mut Vec<SceneEvent>) {
        for element in self.elements.iter_mut() {
            element.click(position, &mut self.events);
        }
    }

    fn get_pos(&self) -> &Position {
        &self.position
    }
    
    fn manage_events(&mut self, events: &mut Vec<SceneEvent>){
        for event in self.events.iter() {
            match event {
                SceneEvent::BoardEvent(BoardEvent::BoardToggleIdleMove(space)) => {
                    let status = self.elements[0].get_status();
                    let status = Board::toggle_status(status, &space);
                    if let Some(s) = status {
                        self.elements[0].set_status(s);
                    }
                },
                SceneEvent::BoardEvent(BoardEvent::BoardSelectMove(space)) => {
                    self.elements[0].set_status(BoardStatus::SELECTING_MOVE(Some(space.clone())));
                },
                _ => {events.push(event.clone());}
            }
        }
        self.events = Vec::new();
    }
}