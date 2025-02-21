use crate::{
    enums::event::SceneEvent, models::{
        fighter::Fighter, 
        position::Position, 
        scenes::{
            scene::Scene, 
            scene_creation_data::SceneCreationData
        }
    }, render_logic::modal_scene_render_logic::ModalSceneRenderLogic, traits::game_object::GameObject
};

use super::character_menu::CharacterMenu;


pub struct Modal {}

impl Modal {
    pub fn new(fighter: Fighter) -> Scene {
        let mut elements: Vec<Box<dyn GameObject<SceneEvent>>> = Vec::new();
        elements.push(Box::new(CharacterMenu::new_for_fighter(&fighter)));
        Scene::new_from_data( SceneCreationData {
            position: Some(Position::default()),
            renderer: Some(Box::new(ModalSceneRenderLogic { _fighter: fighter })),
            elements: Some(elements),
            events: None
        })
    }
}