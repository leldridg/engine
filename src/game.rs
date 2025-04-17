use crate::object::Object;

pub trait Game {
    fn initialize(&mut self);
    fn update(&mut self);
    fn get_objects(&mut self) -> Vec<&Object>;
    fn get_projection_matrix(&self) -> glam::Mat4;
    fn get_view_matrix(&self) -> glam::Mat4;
    fn handle_event(&mut self, event: sdl2::event::Event);
}