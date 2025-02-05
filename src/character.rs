use macroquad::{prelude::*, ui::{hash, root_ui}};

use crate::{
    character_menu::CharacterMenu, 
    clickable::{Clickable, Position, Positionable}};

#[derive(Debug, Clone)]
pub struct Character {
    pub position: Position,
    toggled: bool,
    menu: CharacterMenu,
}

impl Character {
    pub fn new() -> Character {
        Character { 
            position: Position {
                x: 0.0, y: 0.0, w: 0.0, h: 0.0,
            },
            toggled: false,
            menu: CharacterMenu::new(),
        }
    }

    pub fn draw(&self) {
        let mut color = PINK;
        if (self.toggled) {color = BLACK};
        draw_rectangle(self.get_x() + 20.0, self.get_y() + 20.0, self.get_width() - 40.0, self.get_height() - 40.0, color);
        if self.toggled {
            println!("dibujando ventana");
             /*
        Scene::push_modal(*/
            root_ui().window(
            hash!(), // Identificador único para la ventana
            Vec2::new(100.0, 100.0),
            Vec2::new(200.0, 500.0),
            |ui| {
                // Contenido del modal
                ui.label(None, "¡Este es un modal!");
                ui.label(None, "Haz clic fuera para cerrar");

            },
            );/*
        )
        */
        }
    }
}

impl Positionable for Character {
    fn get_pos(&self) -> &Position {
        &self.position   
    }
}

impl Clickable for Character {
    fn click_action(&mut self, x: f32, y: f32) {
        self.toggled = !&self.toggled;
    }
}