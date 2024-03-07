use crate::position::SubGridPos;

pub trait Entity {
    fn spawn(&mut self);
    fn update(&mut self);
    fn render(&self);
    fn get_pos(&self) -> SubGridPos;
}
