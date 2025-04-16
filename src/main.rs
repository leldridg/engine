use std::time::Instant;
use std::collections::HashMap;

use game::Game;
use sdl2::event::{Event, WindowEvent};
use glam::{Mat4, Vec3, Vec4};

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
// window
    let mut width: usize = 600;
    let mut height: usize = 600;
    let mut winsdl = Winsdl::new(width, height).unwrap();
    unsafe { gl::Viewport(0, 0, width as i32, height as i32); }

    let program = create_program().unwrap();
    program.set();

    let u_resolution = Uniform::new(program.id(), "u_resolution").unwrap();
    let u_model_matrix = Uniform::new(program.id(), "u_model_matrix").unwrap();
    let u_view_matrix = Uniform::new(program.id(), "u_view_matrix").unwrap();
    let u_projection_matrix = Uniform::new(program.id(), "u_projection_matrix").unwrap();
    let u_color = Uniform::new(program.id(), "u_color").unwrap();

    // INITIALIZE GAME
    let mut game: Box<dyn Game> = Box::new(Dropper {
        objects: HashMap::new(),
        projection_matrix: Mat4::IDENTITY,
        view_matrix: Mat4::IDENTITY,
    });

    unsafe { 
        gl::Uniform2f(u_resolution.id, width as f32, height as f32);
        gl::UniformMatrix4fv(u_projection_matrix.id, 1, gl::FALSE, projection_matrix.to_cols_array().as_ptr());
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let mut last_frame_time = Instant::now();

    'running: loop {

        game.update();
        render(game.get_objects(), &u_model_matrix, &u_color);

        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(new_width, new_height) = win_event {
                        width = new_width as usize;
                        height = new_height as usize;
                        unsafe {
                            gl::Viewport(0, 0, width as i32, height as i32);
                            gl::Uniform2f(u_resolution.id, width as f32, height as f32);
                        }
                    }
                },
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                            println!("Screen x: {}, Screen y: {}", x, y);
                            // normalize x and y coordinates
                            let norm = Vec4::new(
                                (2. * x as f32) / width as f32 - 1.,
                                1. - (2. * y as f32) / height as f32,
                                -1.,
                                1.
                            );

                            // inverse projection matrix
                            let inverse_projection = projection_matrix.inverse();

                            // multiply inverse projection matrix and (x_n, y_n, z, w)
                            let mut ray_eye = inverse_projection * norm;
                            ray_eye[2] = -1.;
                            ray_eye[3] = 0.;

                            // inverse view matrix
                            let inverse_view = view_matrix.inverse();

                            // multiply inverse view matrix and ray_eye
                            let ray_world = inverse_view * ray_eye;

                            if (perspective) {
                                // normalize ray_world
                                let ray_direction = Vec3::new(ray_world[0], ray_world[1], ray_world[2]).normalize();

                                // use current z value for cube
                                let cube_z = cube_center[2];

                                // calculate the intersection point at the cube's z value
                                let t = (cube_z - eye_z) / ray_direction[2];
                                let world_x = eye_x + t * ray_direction[0];
                                let world_y = eye_y + t * ray_direction[1];

                                println!("World x: {}, World y: {}, World z: {}", world_x, world_y, cube_z);
                                cube_center = Vec3::new(world_x, world_y, cube_z);
                            } else {
                                println!("World x: {}, World y: {}, World z: {}", ray_world[0], ray_world[1], cube_center.z);
                                cube_center = Vec3::new(ray_world[0], ray_world[1], cube_center.z);
                            }
                            cube.set_model_matrix(Mat4::from_translation(cube_center));
                        },
                        _ => { }
                    }
                },
                Event::KeyDown { keycode, .. } => {
                    let amt = 0.1;
                    match keycode {
                        Some(key) => {
                            match key {
                                // cube movement
                                sdl2::keyboard::Keycode::W => {
                                    cube_center[2] -= 0.1;
                                    cube.set_model_matrix(Mat4::from_translation(cube_center));
                                },
                                sdl2::keyboard::Keycode::S => { 
                                    cube_center[2] += 0.1;
                                    cube.set_model_matrix(Mat4::from_translation(cube_center));

                                },
                                sdl2::keyboard::Keycode::A => {
                                    cube_center[0] -= 0.1;
                                    cube.set_model_matrix(Mat4::from_translation(cube_center)); 
                                },
                                sdl2::keyboard::Keycode::D => { 
                                    cube_center[0] += 0.1;
                                    cube.set_model_matrix(Mat4::from_translation(cube_center)); 
                                },
                                sdl2::keyboard::Keycode::Equals => {
                                    cube_center[1] += 0.1;
                                    cube.set_model_matrix(Mat4::from_translation(cube_center));
                                },
                                sdl2::keyboard::Keycode::Minus => {
                                    cube_center[1] -= 0.1;
                                    cube.set_model_matrix(Mat4::from_translation(cube_center));
                                },
                                _ => { }
                            }
                            unsafe {
                                gl::UniformMatrix4fv(u_projection_matrix.id, 1, gl::FALSE, projection_matrix.to_cols_array().as_ptr());
                            }
                            view_matrix = Mat4::look_at_rh(Vec3::new(eye_x, eye_y, eye_z), Vec3::new(target_x, target_y, target_z), Vec3::new(up_x, up_y, up_z));
                        },
                        None => { }
                    }
                }
                _ => { },            
            }
        }
        // apply gravity
        // cube_center += gravity * delta_time;
        


        winsdl.window.gl_swap_window(); // update display
    }
}

fn render(objects: Vec<&Object>, u_model_matrix: &Uniform, u_color: &Uniform) {
    // Render the game
    unsafe {
        gl::ClearColor(54./255., 159./255., 219./255., 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
    

    for object in objects {
        object.render(u_model_matrix, u_color);
    }
}