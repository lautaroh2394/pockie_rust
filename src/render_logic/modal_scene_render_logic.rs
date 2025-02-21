use macroquad::{math::Vec2, ui::{hash, root_ui, Ui}};
use crate::{
    models::{fighter::Fighter, scenes::scene::Scene}, 
    traits::{game_object::GameObject, render_logic::RenderLogic}
};

pub struct ModalSceneRenderLogic { pub fighter: Fighter }
impl RenderLogic<Scene> for ModalSceneRenderLogic {
    fn is_modal(&self) -> bool { true }

    fn render(&self, scene: &Scene){
        for element in scene.elements.iter() {
            element.draw();
        }
        /*
        let x = self.fighter.get_pos().x + self.fighter.get_pos().w - 15.;
        let y = self.fighter.get_pos().y + 15.;

        root_ui().window(
            hash!(),
            Vec2::new(x, y),
            Vec2::new(150., 250.),
            |ui: &mut Ui| {
                    ui.label(None, self.fighter.get_name().as_str());
                    if ui.button(None, "click me") {
                        println!("hi");
                    }
                    let mut owned_string: String = "hello ".to_owned();
                    let borrowed_string: &str = "world";
                    owned_string.push_str(borrowed_string);

                    let mut atk_full_desc: String = "atk ".to_string();
                    let atk = self.fighter.get_atk();
                    let atk_desc = atk.to_string();
                    atk_full_desc.push_str(atk_desc.as_str());

                    let mut def_full_desc: String = "def ".to_string();
                    let def = self.fighter.get_def();
                    let def_desc = def.to_string();
                    def_full_desc.push_str(def_desc.as_str());

                    let mut mv_full_desc: String = "mv ".to_string();
                    let mv = self.fighter.get_movement();
                    let mv_desc = mv.to_string();
                    mv_full_desc.push_str(mv_desc.as_str());

                    ui.label(None, &atk_full_desc);
                    ui.label(None, &def_full_desc);
                    ui.label(None, &mv_full_desc);
            }
        );
         */
    }
}
