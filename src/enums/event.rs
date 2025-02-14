use crate::models::space::Space;


pub enum Event {
    CreateModal,
    BoardIdle,
    BoardSelectVictim,
    BoardSelectMove(Space),
}