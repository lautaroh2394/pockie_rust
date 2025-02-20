use crate::{models::scenes::scene::Scene, traits::render_logic::RenderLogic};

pub struct DefaultSceneRenderLogic {}
impl RenderLogic<Scene> for DefaultSceneRenderLogic {
    fn render(&self, scene: &Scene){
        for element in scene.elements.iter() {
            element.draw();
        }
    }
}