use crate::{
    models::{fighter::Fighter, scenes::scene::Scene}, 
    traits::render_logic::RenderLogic
};

pub struct ModalSceneRenderLogic { pub _fighter: Fighter }
impl RenderLogic<Scene> for ModalSceneRenderLogic {
    fn is_modal(&self) -> bool { true }

    fn render(&self, scene: &Scene){
        for element in scene.elements.iter() {
            element.draw();
        }
    }
}
