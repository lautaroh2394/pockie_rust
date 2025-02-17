use macroquad::{math::Vec2, ui::{hash, root_ui, Ui}};

use crate::{enums::event::SceneEvent, models::scene::Scene, traits::game_object::GameObject};

pub trait RenderLogic<T: GameObject<SceneEvent>> {
    fn render(&self, object: &T){
        object.draw();
    }
}

pub struct DefaultSceneRenderLogic {}
impl RenderLogic<Scene> for DefaultSceneRenderLogic {
    fn render(&self, scene: &Scene){
        for element in scene.elements.iter() {
            element.draw();
        }
    }
}

pub struct ModalSceneRenderLogic {}
impl RenderLogic<Scene> for ModalSceneRenderLogic {
    fn render(&self, _scene: &Scene){
        root_ui().window(
            hash!(),
            Vec2::new(100.0, 100.0),
            Vec2::new(200.0, 500.0),
            |ui: &mut Ui| {
                    ui.label(None, "¡Este es un modal!");
                    ui.label(None, "Haz clic fuera para cerrar");
            }
        );
    }
}
