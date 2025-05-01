use glam::{Mat4, Vec3};

use super::graphics::*;

#[derive(Clone)]
pub struct Object {
    vbo: Vbo,
    vao: Vao,
    ibo: Ibo,
    model_matrix: Mat4,
    center: Vec3,
    index_count: i32,
    vertices: Vec<Vec3>, // Axis-Aligned Bounding Box
    color: Vec3
}

impl Object {
    pub fn new(vertices: &Vec<Vec3>, indices: &Vec<u32>, center: Vec3, color: Vec3) -> Self {

        let vbo = Vbo::gen();
        vbo.set(vertices);

        let vao = Vao::gen();
        vao.set();

        let ibo = Ibo::gen();
        ibo.set(indices);

        let model_matrix = Mat4::from_translation(center);

        Object {
            vbo,
            vao,
            ibo,
            model_matrix,
            center,
            index_count: indices.len() as i32,
            vertices: vertices.clone(), // Assuming object is a cube, rectangular prism, or plane with no rotation
            color,
        }
    }

    pub fn set_model_matrix(&mut self, matrix: Mat4) {
        self.model_matrix = matrix;
        self.center = matrix.w_axis.truncate();
    }

    pub fn get_center(&self) -> Vec3 {
        self.center
    }

    pub fn get_aabb(&self) -> Vec<Vec3> {
        let mut aabb = vec![];
        for vertex in &self.vertices {
            let transformed_vertex = self.model_matrix * vertex.extend(1.0);
            aabb.push(transformed_vertex.truncate());
        }
        aabb
    }

    pub fn render(&self, u_model_matrix: &Uniform, u_color: &Uniform) {
        unsafe {
            gl::UniformMatrix4fv(u_model_matrix.id, 1, gl::FALSE, self.model_matrix.to_cols_array().as_ptr());
            gl::Uniform3fv(u_color.id, 1, self.color.to_array().as_ptr());
            self.vao.bind();
            gl::DrawElements(gl::TRIANGLES, self.index_count, gl::UNSIGNED_INT, 0 as *const _,);
        }
    }

    pub fn intersect(&self, other: &Object) -> bool {
        let mut self_min_x = self.get_aabb()[0].x;
        let mut self_max_x = self.get_aabb()[0].x;
        let mut self_min_y = self.get_aabb()[0].y;
        let mut self_max_y = self.get_aabb()[0].y;
        let mut self_min_z = self.get_aabb()[0].z;
        let mut self_max_z = self.get_aabb()[0].z;

        for i in 1..self.get_aabb().len() {
            if self.get_aabb()[i].x < self_min_x {
                self_min_x = self.get_aabb()[i].x;
            }
            if self.get_aabb()[i].x > self_max_x {
                self_max_x = self.get_aabb()[i].x;
            }
            if self.get_aabb()[i].y < self_min_y {
                self_min_y = self.get_aabb()[i].y;
            }
            if self.get_aabb()[i].y > self_max_y {
                self_max_y = self.get_aabb()[i].y;
            }
            if self.get_aabb()[i].z < self_min_z {
                self_min_z = self.get_aabb()[i].z;
            }
            if self.get_aabb()[i].z > self_max_z {
                self_max_z = self.get_aabb()[i].z;
            }
        }

        let mut other_min_x = other.get_aabb()[0].x;
        let mut other_max_x = other.get_aabb()[0].x;
        let mut other_min_y = other.get_aabb()[0].y;
        let mut other_max_y = other.get_aabb()[0].y;
        let mut other_min_z = other.get_aabb()[0].z;
        let mut other_max_z = other.get_aabb()[0].z;

        for i in 1..other.get_aabb().len() {
            if other.get_aabb()[i].x < other_min_x {
                other_min_x = other.get_aabb()[i].x;
            }
            if other.get_aabb()[i].x > other_max_x {
                other_max_x = other.get_aabb()[i].x;
            }
            if other.get_aabb()[i].y < other_min_y {
                other_min_y = other.get_aabb()[i].y;
            }
            if other.get_aabb()[i].y > other_max_y {
                other_max_y = other.get_aabb()[i].y;
            }
            if other.get_aabb()[i].z < other_min_z {
                other_min_z = other.get_aabb()[i].z;
            }
            if other.get_aabb()[i].z > other_max_z {
                other_max_z = other.get_aabb()[i].z;
            }
        }

        self_min_x <= other_max_x &&
        self_max_x >= other_min_x &&
        self_min_y <= other_max_y &&
        self_max_y >= other_min_y &&
        self_min_z <= other_max_z &&
        self_max_z >= other_min_z
    }

}