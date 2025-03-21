use macroquad::{color::GRAY, shapes::draw_rectangle, window::screen_width};

use crate::{enums::event::SceneEvent, global_events::push_global_event, models::{buttons::{attack_button::AttackButton, button::Button, move_button::MoveButton}, fighter::Fighter, position::Position}, traits::game_object::GameObject};

pub struct CharacterMenu {
    position: Position,
    options: Vec<Button>,
    fighter: Fighter,
}

impl CharacterMenu {
    pub fn new_for_fighter(fighter: &Fighter) -> Self {
        let options = vec![String::from("Move"), String::from("Attack")];
        let mut pos = fighter.get_pos().clone();
        pos.x += 50.;
        pos.y += 50.;
        pos.h = options.len() as f32 * 40. + 20.; // 10px top y bot + 40px por option (cada option tiene 30 del buton y 10 de padding abajo)
        
        let board_width = screen_width() * 0.8;
        let space_width = board_width / 9. as f32;
        pos.w = space_width;
        
        let mut m = CharacterMenu {
            position: pos,
            options: Vec::new(),
            fighter: fighter.clone()
        };
        
        m.add_options(options);
        m
    }

    pub fn add_options(&mut self, options_config: Vec<String>) {
        let option_height = 30.;
        let option_width = self.get_width() - (10. * 2.);
        let x = self.get_x() + 10.;

        for (i, option_title) in options_config.iter().enumerate() {
            if (option_title == "Move") {
                self.options.push(
                    MoveButton::new(
                        Position {
                            x,
                            w: option_width,
                            h: option_height,
                            y: self.get_y() + 10. + (40. * i as f32)
                        },
                        self.fighter.clone()
                ));
            }
            
            if (option_title == "Attack") {
                self.options.push(
                    AttackButton::new(
                        Position {
                            x,
                            w: option_width,
                            h: option_height,
                            y: self.get_y() + 10. + (40. * i as f32)
                        },
                        self.fighter.clone()
                ));
            }
        }


    } 
}

impl GameObject for CharacterMenu {
    fn draw(&self){
        draw_rectangle(
            self.get_x(), 
            self.get_y(),  
            self.get_width(), 
            self.get_height(),
             GRAY
        );

        for button in self.options.iter() {
            button.draw();
        }
    }

    fn click_action(&mut self, position: &Position){
        for option in self.options.iter_mut() {
            option.click(position);
        }
    }
    
    fn get_pos(&self) -> &Position {
        &self.position
    }

    fn click(&mut self, position: &Position) -> bool {
        if self.is_clicked(position) {
            self.click_action(position);
            return true;
        }
        push_global_event(SceneEvent::pop_last());
        false
    }
}