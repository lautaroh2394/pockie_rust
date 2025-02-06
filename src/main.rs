use std::{cell::RefCell, rc::Rc};

use character::Character;
use clickable::Clickable;
use drawable::Drawable;
use macroquad::{prelude::*, ui::{hash, root_ui}};
mod space;
mod character;
mod board;
mod window_conf;
mod clickable;
mod character_menu;
mod scene;
mod scene_manager;
mod drawable;
mod events;
use board::Board;
use scene_manager::SceneManager;
use scene::Scene;
use window_conf::default_conf;

#[macroquad::main(default_conf)]
async fn main() {
    let manager: SceneManager = SceneManager::new();
    let ref_to_manager = Rc::new(RefCell::new(manager));
    //let r2 = ref_to_manager.clone();
    //let mut r2b = r2.borrow_mut();

    let mut board: Board = Board::new();
    let c = Character::new();
    board.set_character_to_coordinate(c, 4, 5);
    //let selfref = Rc::new(RefCell::new(self));
    let mut scene = Scene::new_full_screen(ref_to_manager.clone());
    scene.push(board);
    ref_to_manager.clone().borrow_mut().push(scene);
    //r2b.new_testing_scene();
    //
    /*
*/
    loop {
        clear_background(RED);
        let r3 =          ref_to_manager.clone();
        let mut mutref = r3.borrow_mut();
        mutref.draw();
        mutref.manage_events();

        if is_key_down(KeyCode::Escape) {
            println!("Escape");
            break;
        }

        if is_mouse_button_released(MouseButton::Left){
            let (x, y) = mouse_position();
            println!("Left click released, Mouse pos: {x}, {y}");
            mutref.click(x, y);
        }

        draw_text(&(get_fps().to_string()), 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
    
}