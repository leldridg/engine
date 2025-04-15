pub trait Game {
    fn initialize(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn handle_event(&mut self, event: sdl2::event::Event);
}