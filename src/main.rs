use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;
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
mod screenable;

use scene_manager::SceneManager;
use window_conf::default_conf;

#[macroquad::main(default_conf)]
async fn main() {
    let manager: SceneManager = SceneManager::new();
    let manager_ref = Rc::new(RefCell::new(manager));
    SceneManager::new_testing_scene(manager_ref.clone());
    let manager_ref_2 = manager_ref.clone();
    
    loop {
        clear_background(DARKGRAY);

        let m1 = manager_ref.clone();
        let m2 = manager_ref.clone();
        let m3 = manager_ref.clone();

        {
            let mut r1 = m1.borrow_mut();
            r1.draw();
        }

        {
            let mut r2 = m2.borrow_mut(); // no puedo tener dos ref mut al mismo tiempo
            //r2.manage_events(m3);
        }

        //manager_ref.clone().borrow_mut().manage_events();
        //manager_ref.clone().borrow_mut().draw();

        if is_key_down(KeyCode::Escape) {
            println!("Escape");
            break;
        }

        if is_mouse_button_released(MouseButton::Left){
            let (x, y) = mouse_position();
            println!("Left click released, Mouse pos: {x}, {y}");
            manager_ref.clone().borrow_mut().click(x, y);
        }

        draw_text(&(get_fps().to_string()), 20.0, 20.0, 30.0, DARKGRAY);
        next_frame().await
    }
    
}