use crate::object::Object;

pub trait Game {
    fn initialize(&mut self);
    fn update(&mut self, delta_time: f32);
    fn get_objects(&self) -> Vec<(String, &Object)>;
    fn get_projection_matrix(&self) -> glam::Mat4;
    fn get_view_matrix(&self) -> glam::Mat4;
    fn handle_event(&mut self, event: sdl2::event::Event);
    fn handle_collisions(&mut self, collisions: Vec<(String, String)>); // <Object name, Object name>
}