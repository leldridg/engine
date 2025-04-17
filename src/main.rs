use std::collections::HashMap;

use game::Game;
use sdl2::event::Event;
use glam::Mat4;

mod winsdl;
use winsdl::Winsdl;

mod graphics;
use graphics::*;

mod dropper;
use dropper::Dropper;

mod object;
use object::Object;

mod game;

fn main() {
    // CREATE WINDOW
    // change so this is game dependent?
    let width: usize = 600;
    let height: usize = 600;
    let mut winsdl = Winsdl::new(width, height).unwrap();
    unsafe { gl::Viewport(0, 0, width as i32, height as i32); }

    // CREATE PROGRAM
    let program = create_program().unwrap();
    program.set();

    // CREATE UNIFORMS
    let u_resolution = Uniform::new(program.id(), "u_resolution").unwrap();
    let u_model_matrix = Uniform::new(program.id(), "u_model_matrix").unwrap();
    let u_view_matrix = Uniform::new(program.id(), "u_view_matrix").unwrap();
    let u_projection_matrix = Uniform::new(program.id(), "u_projection_matrix").unwrap();
    let u_color = Uniform::new(program.id(), "u_color").unwrap();

    // CREATE GAME INSTANCE
    let mut game: Box<dyn Game> = Box::new(Dropper {
        objects: HashMap::new(),
        projection_matrix: Mat4::IDENTITY,
        view_matrix: Mat4::IDENTITY,
        screen_width: width as f32,
        screen_height: height as f32,
    });

    // INITIALIZE GAME
    game.initialize();

    unsafe { 
        // SET RESOLUTION
        gl::Uniform2f(u_resolution.id, width as f32, height as f32);

        // SET DEPTH HANDLE
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    // let mut last_frame_time = Instant::now();

    'running: loop {

        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => { game.handle_event(event); },            
            }
        }

        game.update();

        //RENDER
        unsafe {
            // CLEAR W/ BGRD COLOR
            gl::ClearColor(54./255., 159./255., 219./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // SET FRUSTUM
            gl::UniformMatrix4fv(u_projection_matrix.id, 1, gl::FALSE, game.get_projection_matrix().to_cols_array().as_ptr());
            // SET VIEW
            gl::UniformMatrix4fv(u_view_matrix.id, 1, gl::FALSE, game.get_view_matrix().to_cols_array().as_ptr());
        }
    
        render(game.get_objects(), &u_model_matrix, &u_color);

        winsdl.window.gl_swap_window(); // update display
    }
}

fn render(objects: Vec<&Object>, u_model_matrix: &Uniform, u_color: &Uniform) {
    // Render the game objects
    for object in objects {
        object.render(u_model_matrix, u_color);
    }
}