use std::{cell::RefCell, rc::Rc};

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
    let manager = Rc::new(RefCell::new(SceneManager::new()));
    manager.borrow_mut().new_testing_scene();

    loop {
        let mut mutref =         manager.borrow_mut();
        mutref.draw();
        mutref.manage_events();

        if is_key_down(KeyCode::Escape) {
            println!("Escape");
            break;
        }

        if is_mouse_button_released(MouseButton::Left){
            let (x, y) = mouse_position();
            println!("Left click released, Mouse pos: {x}, {y}");
            //ref_to_manager.borrow_mut().click(x, y);
        }

        clear_background(RED);
        let new_ref = manager.clone();
        //mutref.manage_events();
        draw_text(&(get_fps().to_string()), 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}