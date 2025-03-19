use crate::enums::event::SceneEvent;

pub static mut GLOBAL_EVENTS: Vec<SceneEvent> = Vec::new();
pub static mut REMOVABLE_EVENTS: Vec<u64> = Vec::new();

pub fn push_global_event(ev: SceneEvent){
    unsafe {
        GLOBAL_EVENTS.push(ev);
    }
}

pub fn remove_global_event_by_id(id: u64){
    unsafe {
        REMOVABLE_EVENTS.push(id);
    }
}

pub fn remove_global_events(){
    unsafe {
        for id in REMOVABLE_EVENTS.iter() {
            let found = GLOBAL_EVENTS.iter().enumerate().find(|(index, event)|{
                event.id() == *id
            });
            if let Some((index, _)) = found {
                GLOBAL_EVENTS.remove(index);
            }
        }
        REMOVABLE_EVENTS.clear();
    }
}

pub fn global_events_iterate_mut<T>(mut callback: T) 
    where T: FnMut(&SceneEvent) -> ()
    {
        unsafe {
            for event in GLOBAL_EVENTS.iter() {
                callback(event);
            }
        }
}