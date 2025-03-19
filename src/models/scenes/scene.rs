
use crate::{
    enums::event::SceneEvent, models::position::Position, traits::{
        game_object::GameObject, 
        render_logic::RenderLogic,
    },
    render_logic::default_scene_render_logic::DefaultSceneRenderLogic, 
};

use super::scene_creation_data::SceneCreationData;

pub struct Scene {
    pub elements: Vec<Box<dyn GameObject>>,
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
    
    pub fn push(&mut self, element: Box<dyn GameObject>){
        self.elements.push(element);
    }
    
    pub fn is_modal(&self) -> bool { self.renderer.is_modal() }
}

impl GameObject for Scene {
    fn draw(&self) {
        self.renderer.render(self);
    }

    fn click_action(&mut self, position: &Position) {
        for element in self.elements.iter_mut() {
            element.click(position);
        }
    }

    fn get_pos(&self) -> &Position {
        &self.position
    }
    
    fn manage_events(&mut self){
        for object in self.elements.iter_mut() {
            object.manage_events();
        }
    }
}