use std::time::Instant;

use sdl2::event::{Event, WindowEvent};
use glam::{Mat4, Vec3, Vec4, Quat};

mod winsdl;
use winsdl::Winsdl;

mod objects;
use objects::*;

mod graphics;
use graphics::*;

fn main() {
// window
    let mut width: usize = 600;
    let mut height: usize = 600;
    let mut winsdl = Winsdl::new(width, height).unwrap();
    unsafe { gl::Viewport(0, 0, width as i32, height as i32); }
    
    let mut max_uniforms: gl::types::GLint = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_UNIFORM_VECTORS, &mut max_uniforms); }
    println!("Max uniforms: {}", max_uniforms);
    println!("Maximum number of uniforms: {}", std::mem::size_of::<Vec3>());

    let mut orthographic: bool = false;
    let mut perspective: bool = false;

    let program = create_program().unwrap();
    program.set();

    let plane_vertices: Vec<Vec3> = vec! [
        Vec3::new(-1., 0., 1.),
        Vec3::new(-1., 0., -1.),
        Vec3::new(1., 0., -1.),
        Vec3::new(1., 0.,  1.),
    ];

    let plane_indices: Vec<u32> = vec! [
        0, 1, 2,
        2, 3, 0,
    ];

    let cube_vertices: Vec<Vec3> = vec! [
        // front face
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(0.5, -0.5, -0.5),

        // back face
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(-0.5, -0.5, 0.5),
    ];

    let cube_indices: Vec<u32> = vec! [
        // front face
        0, 1, 2,
        2, 3, 0,

        // back face
        4, 5, 6,
        6, 7, 4,

        // top face
        1, 6, 5,
        5, 2, 1,

        // bottom face
        7, 0, 3,
        3, 4, 7,

        // right face
        3, 2, 5,
        5, 4, 3,

        // left face
        7, 6, 1,
        1, 0, 7,
    ];

    let mut cube_center = Vec3::new(0.,0.,0.);

// objects to render

    let mut cube = Object::new(&cube_vertices, &cube_indices, cube_center, Vec3::new(1., 0., 0.));
    //let cube2 = Object::new(&cube_vertices, &cube_indices, Vec3::new(0.,0.,0.), Vec3::new(0., 1., 0.));
    let mut plane = Object::new(&plane_vertices, &plane_indices, Vec3::new(0.,0.,0.), Vec3::new(0., 0., 1.));
    plane.set_model_matrix(Mat4::from_scale_rotation_translation(Vec3::new(5., 1., 5.), Quat::IDENTITY, Vec3::new(0., -3., 0.)));


// view and projection matrices

    let mut view_matrix: Mat4;
    let mut projection_matrix: Mat4 = Mat4::IDENTITY;

// values for projection

    let l = -10.;
    let r = 10.;
    let b = -10.;
    let t = 10.;
    let n = 0.1;
    let f = 100.;

    let aspect_ratio = (r - l) / (t - b);
    println!("Aspect ratio is {} for r {}, l {}, t {}, and b {}", aspect_ratio, r, l, t, b);

    let fov_y=(2.0 * ((t - b) / (2.0 * n)) as f32).atan();
    println!("Fov_y is {} for t {}, b {}, n {}", fov_y, t, b, n);

    let u_resolution = Uniform::new(program.id(), "u_resolution").unwrap();
    let u_model_matrix = Uniform::new(program.id(), "u_model_matrix").unwrap();
    let u_view_matrix = Uniform::new(program.id(), "u_view_matrix").unwrap();
    let u_projection_matrix = Uniform::new(program.id(), "u_projection_matrix").unwrap();
    let u_color = Uniform::new(program.id(), "u_color").unwrap();

    println!("u_color location: {}", u_color.id);

    unsafe { 
        gl::Uniform2f(u_resolution.id, width as f32, height as f32);
        gl::UniformMatrix4fv(u_projection_matrix.id, 1, gl::FALSE, projection_matrix.to_cols_array().as_ptr());
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let mut eye_x = 0.0;
    let mut eye_y = 0.0;
    //let mut eye_z = 5.;
    let mut eye_z = 5.0;
    let target_x = 0.;
    let target_y = 0.;
    let mut target_z = 0.;
    let up_x = 0.;
    let mut up_y = 1.;
    let up_z = 0.;

    view_matrix = Mat4::look_at_rh(Vec3::new(eye_x, eye_y, eye_z), Vec3::new(target_x, target_y, target_z), Vec3::new(up_x, up_y, up_z));

    let gravity = Vec3::new(0., -0.5, 0.);

    let mut last_frame_time = Instant::now();

    'running: loop {
    
        let current_frame_time = Instant::now();
        let delta_time = current_frame_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_frame_time;

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
                                // camera movement
                                sdl2::keyboard::Keycode::Left => { 
                                    eye_x -= amt;
                                },
                                sdl2::keyboard::Keycode::Right => {
                                    eye_x += amt;
                                },
                                sdl2::keyboard::Keycode::Up => {
                                    eye_z -= amt;
                                    //up_y -= amt;
                                },
                                sdl2::keyboard::Keycode::Down => {
                                    eye_z += amt;
                                    //up_y += amt;
                                },
                                sdl2::keyboard::Keycode::Comma => {
                                    eye_y -= amt;
                                },
                                sdl2::keyboard::Keycode::Period => {
                                    eye_y += amt;
                                },
                                sdl2::keyboard::Keycode::LeftBracket => {
                                    target_z -= amt;
                                },
                                sdl2::keyboard::Keycode::RightBracket => {
                                    target_z += amt;
                                },
                                // projection
                                sdl2::keyboard::Keycode::P => {
                                    projection_matrix = Mat4::perspective_infinite_rh(fov_y, aspect_ratio, n);
                                    perspective = true;
                                    orthographic = false;
                                },
                                sdl2::keyboard::Keycode::O => {
                                    projection_matrix = Mat4::orthographic_rh_gl(l, r, b, t, n, f);
                                    orthographic = true;
                                    perspective = false;
                                },
                                _ => { }
                            }
                            unsafe {
                                gl::UniformMatrix4fv(u_projection_matrix.id, 1, gl::FALSE, projection_matrix.to_cols_array().as_ptr());
                            }
                            view_matrix = Mat4::look_at_rh(Vec3::new(eye_x, eye_y, eye_z), Vec3::new(target_x, target_y, target_z), Vec3::new(up_x, up_y, up_z));
                            println!("Key pressed: {:?}", key);
                            println!("eye_x: {}, eye_y: {}, eye_z: {}", eye_x, eye_y, eye_z);
                            println!("up_x: {}, up_y: {}, up_z: {}", up_x, up_y, up_z);
                            println!("target_z: {}", target_z);
                            println!("cube_center: {:?}", cube_center);
                            
                        },
                        None => { }
                    }
                }
                _ => { },            
            }
        }
        // apply gravity
        cube_center += gravity * delta_time;
        cube.set_model_matrix(Mat4::from_translation(cube_center));
        unsafe {
            gl::ClearColor(54./255., 159./255., 219./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UniformMatrix4fv(u_view_matrix.id, 1, gl::FALSE, view_matrix.to_cols_array().as_ptr());

            cube.render(&u_model_matrix, &u_color);
            //cube2.render(&u_model_matrix, &u_color);

            plane.render(&u_model_matrix, &u_color);
        }

        winsdl.window.gl_swap_window(); // update display
    }
}