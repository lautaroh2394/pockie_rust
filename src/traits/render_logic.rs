use crate::traits::game_object::GameObject;

pub trait RenderLogic<T: GameObject> {
    fn render(&self, object: &T){
        object.draw();
    }

    fn is_modal(&self) -> bool { false }
}