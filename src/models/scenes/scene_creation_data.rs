use crate::{
    enums::event::SceneEvent, 
    models::position::Position, 
    traits::{game_object::GameObject, render_logic::RenderLogic},
    render_logic::default_scene_render_logic::DefaultSceneRenderLogic,
};

use super::scene::Scene;

pub struct SceneCreationData {
    pub elements: Option<Vec<Box<dyn GameObject<SceneEvent>>>>,
    pub events: Option<Vec<SceneEvent>>,
    pub position: Option<Position>,
    pub renderer: Option<Box<dyn RenderLogic<Scene>>>,
}

impl SceneCreationData {
    pub fn default() -> Self {
        SceneCreationData {
            elements: Some(Vec::new()),
            events: Some(Vec::new()),
            position: Some(Position::default()),
            renderer: Some(Box::new(DefaultSceneRenderLogic {}))
        }
    }
}