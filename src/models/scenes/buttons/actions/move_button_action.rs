use crate::{enums::event::SceneEvent, models::scenes::buttons::button::ButtonAction};

pub struct MoveButtonAction {}
impl ButtonAction for MoveButtonAction {
    fn execute(&self, events: &mut Vec<SceneEvent>) {
        //todo
        println!("move button clicked")
    }
}