use std::collections::HashMap;
use glam::{Mat4, Vec3, Vec4, Quat};
use sdl2::event::Event;

use crate::object::Object;
use crate::game::Game;

pub struct Dropper<'a> {
    pub(crate) objects: HashMap<&'a str, Object>,
    pub(crate) projection_matrix: Mat4,
    pub(crate) view_matrix: Mat4,
    pub(crate) screen_width: f32,
    pub(crate) screen_height: f32,
}

impl<'a> Game for Dropper<'a> {
    fn initialize(&mut self) {
        // Initialize the game

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

        let cube_center = Vec3::new(0.,0.,0.);

        self.objects.insert("cube", Object::new(&cube_vertices, &cube_indices, cube_center, Vec3::new(1., 0., 0.)));

        self.objects.insert("plane", Object::new(&plane_vertices, &plane_indices, Vec3::new(0.,0.,0.), Vec3::new(0., 0., 1.)));

        // Scale and translate plane
        self.objects.get_mut("plane").unwrap().set_model_matrix(Mat4::from_scale_rotation_translation(Vec3::new(5., 1., 5.), Quat::IDENTITY, Vec3::new(0., -3., 0.)));

        let l = -10.;
        let r = 10.;
        let b = -10.;
        let t = 10.;
        let n = 0.1;
        let aspect_ratio = (r - l) / (t - b);
        let fov_y=(2.0 * ((t - b) / (2.0 * n)) as f32).atan();

        self.projection_matrix = Mat4::perspective_infinite_rh(fov_y, aspect_ratio, n);

        let eye_x = 0.0;
        let eye_y = 0.0;
        let eye_z = 5.0;
        let target_x = 0.;
        let target_y = 0.;
        let target_z = 0.;
        let up_x = 0.;
        let up_y = 1.;
        let up_z = 0.;

        self.view_matrix = Mat4::look_at_rh(Vec3::new(eye_x, eye_y, eye_z), Vec3::new(target_x, target_y, target_z), Vec3::new(up_x, up_y, up_z));
    }

    fn update(&mut self) {
        // Update the game

        let cube = self.objects.get_mut("cube").unwrap();
        cube.set_model_matrix(Mat4::from_translation(cube.get_center()));

    }

    fn get_objects(&mut self) -> Vec<&Object> {
        self.objects.iter()
            .map(|(_, object)| object)
            .collect()
    }

    fn get_projection_matrix(&self) -> glam::Mat4 {
        self.projection_matrix
    }

    fn get_view_matrix(&self) -> glam::Mat4 {
        self.view_matrix
    }

    fn handle_event(&mut self, event: sdl2::event::Event) {
        // Handle user input events
        let cube = self.objects.get_mut("cube").unwrap();
        let mut cube_center = cube.get_center();

        match event {
            Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => {
                        // normalize x and y coordinates
                        let norm = Vec4::new(
                            (2. * x as f32) / self.screen_width as f32 - 1.,
                            1. - (2. * y as f32) / self.screen_height as f32,
                            -1.,
                            1.
                        );

                        // inverse projection matrix
                        let inverse_projection = self.projection_matrix.inverse();

                        // multiply inverse projection matrix and (x_n, y_n, z, w)
                        let mut ray_eye = inverse_projection * norm;
                        ray_eye[2] = -1.;
                        ray_eye[3] = 0.;

                        // inverse view matrix
                        let inverse_view = self.view_matrix.inverse();

                        // get camera position
                        let eye_position = inverse_view.col(3).truncate(); // Extract the translation (x, y, z)
                        let eye_x = eye_position.x;
                        let eye_y = eye_position.y;
                        let eye_z = eye_position.z;

                        // multiply inverse view matrix and ray_eye
                        let ray_world = inverse_view * ray_eye;

                        // normalize ray_world
                        let ray_direction = Vec3::new(ray_world[0], ray_world[1], ray_world[2]).normalize();

                        // use current z value for cube
                        let cube_z = cube_center[2];

                        // calculate the intersection point at the cube's z value
                        let t = (cube_z - eye_z) / ray_direction[2];
                        let world_x = eye_x + t * ray_direction[0];
                        let world_y = eye_y + t * ray_direction[1];

                        cube_center = Vec3::new(world_x, world_y, cube_z);

                        cube.set_model_matrix(Mat4::from_translation(cube_center));
                    },
                    _ => { }
                }
            },
            Event::KeyDown { keycode, .. } => {
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
                            _ => { }
                        }
                    },
                    None => { }
                }
            }
            _ => { },            
        }
    }
}