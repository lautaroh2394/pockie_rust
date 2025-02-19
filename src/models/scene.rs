
use macroquad::window::{screen_height, screen_width};
use crate::{
    enums::{
        board_status::BoardStatus, 
        event::{
            BoardEvent, SceneEvent
        }
    }, models::{
        board::Board, 
        position::Position,
        fighter::Fighter, 
        render_logic::{
            modal::ModalSceneRenderLogic,
            default::DefaultSceneRenderLogic,
        },
    }, traits::{
        game_object::GameObject, 
        render_logic::RenderLogic,
    }
};

pub struct Scene {
    pub elements: Vec<Box<dyn GameObject<SceneEvent>>>,
    pub events: Vec<SceneEvent>,
    pub position: Position,
    renderer: Box<dyn RenderLogic<Scene>>,
}

impl Scene {
    pub fn new() -> Self {
        let x = 0.0;
        let y = 0.0;
        let h = screen_height();
        let w = screen_width();
        let p = Position {x, y, w, h};
        Scene {
            elements: Vec::new(),
            events: Vec::new(),
            position: p,
            renderer: Box::new(DefaultSceneRenderLogic {})
        }
    }

    pub fn new_modal(fighter: Fighter) -> Self {
        let x = 0.0;
        let y = 0.0;
        let h = screen_height();
        let w = screen_width();
        let p = Position {x, y, w, h};

        Scene {
            elements: Vec::new(),
            events: Vec::new(),
            position: p,
            renderer: Box::new(ModalSceneRenderLogic { fighter: fighter})
        }
    }
    
    pub fn push(&mut self, element: Box<dyn GameObject<SceneEvent>>){
        self.elements.push(element);
    }

    pub fn manage_events(&mut self, events: &mut Vec<SceneEvent>){
        for event in self.events.iter() {
            match event {
                SceneEvent::BoardEvent(BoardEvent::BoardToggleIdleMove(space)) => {
                    let status = self.elements[0].get_status();
                    let status = Board::toggle_status(status, &space);
                    if let Some(s) = status {
                        self.elements[0].set_status(s);
                    }
                },
                SceneEvent::BoardEvent(BoardEvent::BoardSelectMove(space)) => {
                    self.elements[0].set_status(BoardStatus::SELECTING_MOVE(Some(space.clone())));
                },
                SceneEvent::CreateModal(f) => {
                    events.push(SceneEvent::CreateModal(f.clone()));
                }
                _ => {}
            }
        }
        self.events = Vec::new();
    }

    pub fn is_modal(&self) -> bool { self.renderer.is_modal() }
}

impl GameObject<SceneEvent> for Scene {
    fn draw(&self) {
        self.renderer.render(self);
    }

    fn click_action(&mut self, position: &Position, _: &mut Vec<SceneEvent>) {
        for element in self.elements.iter_mut() {
            element.click(position, &mut self.events);
        }
    }

    fn get_pos(&self) -> &Position {
        &self.position
    }
}