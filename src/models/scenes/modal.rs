use crate::{
    models::{
        fighter::Fighter, 
        position::Position, 
        scenes::{
            scene::Scene, 
            scene_creation_data::SceneCreationData
        }
    }, 
    render_logic::modal_scene_render_logic::ModalSceneRenderLogic,
};


pub struct Modal {}

impl Modal {
    pub fn new(fighter: Fighter) -> Scene {
        Scene::new_from_data( SceneCreationData {
            position: Some(Position::default()),
            renderer: Some(Box::new(ModalSceneRenderLogic { fighter })),
            elements: None,
            events: None
        })
    }
}