use std::collections::HashSet;

use glam::{Mat4, Vec3, Quat};
use crate::object::Object;

pub struct Dropper {
    objects: HashSet<Object>,
    projection_matrix: Mat4,
    view_matrix: Mat4,
}

impl Game for Dropper {
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

        let cube = Object::new("cube", &cube_vertices, &cube_indices, cube_center, Vec3::new(1., 0., 0.));
        self.objects.push(cube);

        let plane = Object::new("plane", &plane_vertices, &plane_indices, Vec3::new(0.,0.,0.), Vec3::new(0., 0., 1.));
        self.objects.push(plane);

        // Scale and translate plane
        plane.set_model_matrix(Mat4::from_scale_rotation_translation(Vec3::new(5., 1., 5.), Quat::IDENTITY, Vec3::new(0., -3., 0.)));

        let l = -10.;
        let r = 10.;
        let b = -10.;
        let t = 10.;
        let n = 0.1;
        let f = 100.;
        let aspect_ratio = (r - l) / (t - b);
        let fov_y=(2.0 * ((t - b) / (2.0 * n)) as f32).atan();

        self.projection_matrix = Mat4::perspective_infinite_rh(fov_y, aspect_ratio, n);

        let mut eye_x = 0.0;
        let mut eye_y = 0.0;
        let mut eye_z = 5.0;
        let target_x = 0.;
        let target_y = 0.;
        let mut target_z = 0.;
        let up_x = 0.;
        let mut up_y = 1.;
        let up_z = 0.;

        self.view_matrix = Mat4::look_at_rh(Vec3::new(eye_x, eye_y, eye_z), Vec3::new(target_x, target_y, target_z), Vec3::new(up_x, up_y, up_z));
    }

    fn update(&mut self) {
        // Update the game

        
        cube.set_model_matrix(Mat4::from_translation(cube_center));

    }

    fn render(&mut self) {
        // Render the game
        unsafe {
            gl::ClearColor(54./255., 159./255., 219./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        

        for object in &self.objects {
            object.render(&self.u_model_matrix, &self.u_color);
        }
    }

    fn handle_event(&mut self, event: sdl2::event::Event) {
        // Handle user input events
    }
}