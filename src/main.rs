#![feature(test)]
#![allow(dead_code)]

use std::rc::Rc;
use std::time;

use nanorand::{WyRand, RNG};

use sdl2::pixels::Color;
use sdl2::rect::Point;

mod camera;
mod flatshapes;
mod math;
mod meshes;
mod object;

use camera::*;
use flatshapes::*;
use math::*;
use meshes::*;
use object::*;

#[cfg(test)]
mod bench;

struct Scene {
    camera: Camera,
    objects: Vec<Object>,
}

enum RenderMode {
    Point,
    Line,
    Triangle,
    FilledTriangle,
    ShadedTriangle,
    MultishadedTriangle,
    Mesh,
}

fn rand_percent(rng: &mut WyRand) -> FP {
    rng.generate_range::<u16>(0, 10000) as FP / 10000.0
}

fn project_vertex(camera: &Camera, point: &Vec3) -> Option<Point> {
    // Switch to camera space
    let point_local = camera.rot * (*point - camera.pos);

    if point_local.z <= camera.viewport.z {
        return None;
    }
    // camera.viewport.z / point_local.z  projects to viewport
    let vp_res = point_local.xy() * camera.viewport.z / point_local.z;
    // camera.viewport.w scales to canvas
    let res = (vp_res + (camera.viewport.xy() / 2.0)) * camera.viewport.w;
    Some(Point::new(res.x as i32, res.y as i32))
}

pub fn main() {
    let render_mode = RenderMode::Mesh; // TODO make this runtime changable

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("graphics demo", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    // sdl_context.mouse().set_relative_mouse_mode(true);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present(); // Leave white canvas while the rest of the program inits

    let mut rng = WyRand::new();
    let mut camera = Camera::new(
        Some(Vec3::new(0.0, 0.0, -10.0)),
        None,
        Some(90u8),
        (800, 600),
    );
    // let mut imp_mesh = import_mesh(Path::new("src/teapot.glb"));
    // imp_mesh.truncate(imp_mesh.len() - (imp_mesh.len() % 3));
    let obj = Object {
        pos: Vec3::new(0.0, 3.0, 0.0),
        rot: Quat::default(),
        scale: 4.0,
        mesh: Rc::new(teapot()),
    };

    loop {
        let fr_start = time::Instant::now();

        camera.process_inputs(&mut event_pump);

        match render_mode {
            RenderMode::Point => put_color(
                &mut canvas,
                Point::new(
                    rng.generate_range::<u32>(1, 800) as i32,
                    rng.generate_range::<u32>(1, 600) as i32,
                ),
                Color::BLACK,
            ),
            RenderMode::Line => draw_line(
                &mut canvas,
                Point::new(400, 300),
                Point::new(
                    rng.generate_range::<u32>(100, 700) as i32,
                    rng.generate_range::<u32>(100, 500) as i32,
                ),
                Color::BLACK,
            ),
            RenderMode::Triangle => {
                draw_triangle(
                    &mut canvas,
                    Point::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                    ),
                    Point::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                    ),
                    Point::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                    ),
                    Color::BLACK,
                );
            }
            RenderMode::FilledTriangle => {
                draw_filled_triangle(
                    &mut canvas,
                    Point::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                    ),
                    Point::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                    ),
                    Point::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                    ),
                    Color::BLACK,
                    Color::GREEN,
                );
            }
            RenderMode::ShadedTriangle => {
                draw_shaded_triangle(
                    &mut canvas,
                    Xyh::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                        rand_percent(&mut rng),
                    ),
                    Xyh::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                        rand_percent(&mut rng),
                    ),
                    Xyh::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                        rand_percent(&mut rng),
                    ),
                    Color::from((
                        rng.generate_range::<u8>(0, 255),
                        rng.generate_range::<u8>(0, 255),
                        rng.generate_range::<u8>(0, 255),
                    )),
                );
            }
            RenderMode::MultishadedTriangle => {
                draw_multishade_triangle(
                    &mut canvas,
                    Point::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                    ),
                    Point::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                    ),
                    Point::new(
                        rng.generate_range::<u32>(100, 700) as i32,
                        rng.generate_range::<u32>(100, 500) as i32,
                    ),
                    Color::RED,
                    Color::GREEN,
                    Color::BLUE,
                );
            }
            RenderMode::Mesh => {
                // camera.rot.normalize();
                canvas.set_draw_color(Color::WHITE);
                canvas.clear();
                obj.render(&mut canvas, &camera);
            }
        };
        canvas.present();
        // Comment out for UNLIMITED FPS!!
        std::thread::sleep(
            time::Duration::from_millis(1000 / 60)
                .checked_sub(fr_start.elapsed())
                .unwrap_or_default(),
        );
        // print!("\r {} FPS", (1.0 / fr_start.elapsed().as_secs_f32()) as u32);
    }
}
