use crate::{enums::board_status::BoardStatus, models::position::Position};

// About ReceivedEvent and OwnEvent:
// When clicked, a game object may want to emit an event. 
// The root scene will pass its events vec to each of its elements when looking for a clicked object.
// In some cases, some element may need to manage its own events, different from the scene events.
// For this case, it may use its own events (Vec<OwnEvent>). The parent game object may need to call a manage_events function.
//
// For every other case, a game object:
//  - wont need to emit an event, therefore wont care about the events vec it receives when clicked
//  - will need to emit an event but it suffices its use case to use the Scene events
//  - wont need to emit an event, and its children game objects might but the scene events are enough (same as case 2)
pub trait GameObject<ReceivedEvent, OwnEvent = ReceivedEvent> {
//pub trait GameObject<ReceivedEvent> {
    fn draw(&self);
    fn click_action(&mut self, position: &Position, events: &mut Vec<ReceivedEvent>);

    fn get_pos(&self) -> &Position;

    fn get_x(&self) -> f32 {
        self.get_pos().x
    }

    fn get_y(&self) -> f32 {
        self.get_pos().y
    }

    fn get_width(&self) -> f32 {
        self.get_pos().w
    }

    fn get_height(&self) -> f32 {
        self.get_pos().h
    }
    
    fn click(&mut self, position: &Position, events: &mut Vec<ReceivedEvent>) -> bool {
        if self.is_clicked(position) {
            self.click_action(position, events);
            return true;
        }
        false
    }

    fn default_click_condition(&self, position: &Position) -> bool {
        let overlaps_x = (self.get_x() <= position.x) && (self.get_x() + self.get_width() >= position.x);
        let overlaps_y = (self.get_y() <= position.y) && (self.get_y() + self.get_height() >= position.y);
        overlaps_x && overlaps_y
    }
    

    fn is_clicked(&self, position: &Position) -> bool{ 
        self.default_click_condition(position)
    }

    fn get_name(&self) -> String { String::from("Nombre sin definir") }
    fn set_status(&mut self, _: BoardStatus){}
    fn get_status(&self) -> &BoardStatus { &BoardStatus::IDLE(None) }
    fn get_events(&self) -> Option<OwnEvent> { None }
    fn manage_events(&mut self, _events: &mut Vec<ReceivedEvent>) {}
}