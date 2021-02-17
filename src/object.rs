use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::camera::*;
use crate::flatshapes::*;
use crate::math::*;
use crate::project_vertex;

pub struct Object {
    pub pos: Vec3,
    pub rot: Quat,
    pub scale: FP,
    pub mesh: Rc<Vec<Vec3>>,
}

impl Object {
    pub fn render(&self, canvas: &mut WindowCanvas, camera: &Camera) {
        assert_eq!(self.mesh.len() % 3, 0);
        for idx in (0..self.mesh.len()).step_by(3) {
            draw_triangle(
                canvas,
                project_vertex(
                    camera,
                    &((self.rot * self.mesh[idx] * self.scale) + self.pos),
                ),
                project_vertex(
                    camera,
                    &((self.rot * self.mesh[idx + 1] * self.scale) + self.pos),
                ),
                project_vertex(
                    camera,
                    &((self.rot * self.mesh[idx + 2] * self.scale) + self.pos),
                ),
                Color::BLACK,
            );
        }
    }
}

pub fn draw_cube(canvas: &mut WindowCanvas, scale: i32, camera: &Camera) {
    let scale = scale as FP;
    let fv_a = Vec3::new(-scale, -scale, 0.0);
    let fv_b = Vec3::new(scale, -scale, 0.0);
    let fv_c = Vec3::new(scale, scale, 0.0);
    let fv_d = Vec3::new(-scale, scale, 0.0);

    let bv_a = Vec3::new(-scale, -scale, scale);
    let bc_b = Vec3::new(scale, -scale, scale);
    let bv_d = Vec3::new(scale, scale, scale);
    let bv_e = Vec3::new(-scale, scale, scale);

    draw_line(
        canvas,
        project_vertex(camera, &fv_a),
        project_vertex(camera, &fv_b),
        Color::BLACK,
    );
    draw_line(
        canvas,
        project_vertex(camera, &fv_b),
        project_vertex(camera, &fv_c),
        Color::BLACK,
    );
    draw_line(
        canvas,
        project_vertex(camera, &fv_c),
        project_vertex(camera, &fv_d),
        Color::BLACK,
    );
    draw_line(
        canvas,
        project_vertex(camera, &fv_d),
        project_vertex(camera, &fv_a),
        Color::BLACK,
    );

    draw_line(
        canvas,
        project_vertex(camera, &bv_a),
        project_vertex(camera, &bc_b),
        Color::BLACK,
    );
    draw_line(
        canvas,
        project_vertex(camera, &bc_b),
        project_vertex(camera, &bv_d),
        Color::BLACK,
    );
    draw_line(
        canvas,
        project_vertex(camera, &bv_d),
        project_vertex(camera, &bv_e),
        Color::BLACK,
    );
    draw_line(
        canvas,
        project_vertex(camera, &bv_e),
        project_vertex(camera, &bv_a),
        Color::BLACK,
    );

    draw_line(
        canvas,
        project_vertex(camera, &fv_a),
        project_vertex(camera, &bv_a),
        Color::BLACK,
    );
    draw_line(
        canvas,
        project_vertex(camera, &fv_b),
        project_vertex(camera, &bc_b),
        Color::BLACK,
    );
    draw_line(
        canvas,
        project_vertex(camera, &fv_c),
        project_vertex(camera, &bv_d),
        Color::BLACK,
    );
    draw_line(
        canvas,
        project_vertex(camera, &fv_d),
        project_vertex(camera, &bv_e),
        Color::BLACK,
    );
}
