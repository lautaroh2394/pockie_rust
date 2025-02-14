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
        
        if is_key_down(KeyCode::Escape) {
            println!("Escape");
            break;
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