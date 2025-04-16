use crate::object::Object;

pub trait Game {
    fn initialize(&mut self);
    fn update(&mut self);
    fn get_objects(&mut self) -> Vec<&Object>;
    fn handle_event(&mut self, event: sdl2::event::Event);
}