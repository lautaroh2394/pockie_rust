use macroquad::prelude::*;
mod window_conf;
mod models;
mod enums;
mod traits;
use models::scene_manager::SceneManager;
use window_conf::default_conf;

#[macroquad::main(default_conf)]
async fn main() {
    let mut manager: SceneManager = SceneManager::new();
    manager.new_testing_scene();
    
    loop {
        clear_background(DARKGRAY);
        manager.manage_events();
        manager.draw();
        
        if is_key_released(KeyCode::Escape) {
            println!("Escape");
            manager.pop_scene();
            if manager.empty() { break }
        }

        if is_mouse_button_released(MouseButton::Left){
            let (x, y) = mouse_position();
            println!("Left click released, Mouse pos: {x}, {y}");
            manager.click(x, y);
        }

        draw_text(&(get_fps().to_string()), 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}