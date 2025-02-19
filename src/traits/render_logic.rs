use crate::{enums::event::SceneEvent, traits::game_object::GameObject};

pub trait RenderLogic<T: GameObject<SceneEvent>> {
    fn render(&self, object: &T){
        object.draw();
    }

    fn is_modal(&self) -> bool { false }
}