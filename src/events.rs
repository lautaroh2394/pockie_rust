use crate::scene_manager::SceneManager;

pub struct EventBus {
    events: Vec<Event>,
}

type Event = Option<Box<dyn Fn(&SceneManager)>>;

impl EventBus {
    fn push(&mut self, event: Event){
        self.events.push(event);
    }

    fn execute_events(&mut self, manager: &SceneManager){
        for event in self.events.iter() {
            if let Some(handler) = event {
                handler(manager);
            };
        }
        self.events = Vec::new();
    }
}