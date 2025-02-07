use std::{cell::RefCell, rc::Rc};

use macroquad::{prelude::*, ui::{hash, root_ui, Ui}};

use crate::{
    character_menu::CharacterMenu, clickable::{Clickable, Position, Positionable}, events::{Event, Modal}, scene::Scene, scene_manager::SceneManager};

#[derive(Debug, Clone)]
pub struct Character {
    pub position: Position,
    toggled: bool,
    _menu: CharacterMenu,
}

impl Character {
    pub fn new() -> Character {
        Character { 
            position: Position {
                x: 0.0, y: 0.0, w: 0.0, h: 0.0,
            },
            toggled: false,
            _menu: CharacterMenu::new(),
        }
    }

    pub fn draw(&self) {
        let mut color = PINK;
        if self.toggled {color = BLACK};
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
    fn click_action<'a>(&mut self, _x: f32, _y: f32, events: &mut Vec<Event>) {
        self.toggled = !&self.toggled;
        let p = Rc::new(RefCell::new(self.position.clone()));
        let new_modal = Modal {
            id: hash!(),
            position: Position {
                x: self.position.x, y: self.position.y, w: self.position.w, h: self.position.h
            },
            f: Box::new(|ui: &mut Ui| {
                ui.label(None, "¡Este es un modal!");
                ui.label(None, "Haz clic fuera para cerrar");
            })
        };
        events.push(Event::CreateModal(new_modal));
    }
}