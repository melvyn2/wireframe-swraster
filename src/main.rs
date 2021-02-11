#![feature(destructuring_assignment)]

use std::time::Duration;

extern crate nanorand;
use nanorand::{RNG, WyRand};

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{WindowCanvas, Canvas};

mod math;
use math::*;

mod camera;
use camera::*;

mod flatshapes;
use flatshapes::*;

struct Mesh<'a> {
    vertices: Vec<Vec3>,
    tris: Vec<(&'a Vec3, &'a Vec3, &'a Vec3)>
}
struct Object<'a> {
    pos: Vec3,
    rot: Quat,
    mesh: Mesh<'a>
}
struct Scene<'a> {
    camera: Camera,
    objects: Vec<Object<'a>>
}

fn rand_percent(rng: &mut WyRand) -> FP {
    rng.generate_range::<u16>(0, 10000) as FP / 10000.0
}

fn project_vertex(camera: &Camera, point: &Vec3) -> Point {
    // Switch to camera space
    let point_local = *point - camera.pos;
    // TODO apply camera rotation

    if point_local.z <= 1.0 {
        return Point::new(0, 0)
    }
    // camera.viewport.z / point_local.z  projects to viewport
    let vp_res = point_local.xy() * camera.viewport.z / point_local.z;
    // camera.viewport.w scales to canvas
    let res = (vp_res + (camera.viewport.xy() / 2.0)) * camera.viewport.w;
    Point::new(res.x as i32, res.y as i32)
}

fn render_object(canvas: &mut WindowCanvas, camera: &Camera, obj: &Object) {
}

fn draw_cube(canvas: &mut WindowCanvas, scale: i32, camera: &Camera) {
    let vAf = Vec3::new(-scale as FP, -scale as FP, 0.0);
    let vBf = Vec3::new(scale as FP, -scale as FP, 0.0);
    let vCf = Vec3::new(scale as FP, scale as FP, 0.0);
    let vDf = Vec3::new(-scale as FP, scale as FP, 0.0);

    let vAb = Vec3::new(-scale as FP, -scale as FP, scale as FP);
    let vBb = Vec3::new(scale as FP, -scale as FP, scale as FP);
    let vCb = Vec3::new(scale as FP, scale as FP, scale as FP);
    let vDb = Vec3::new(-scale as FP, scale as FP, scale as FP);

    draw_line(canvas, project_vertex(camera, &vAf), project_vertex(camera, &vBf), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &vBf), project_vertex(camera, &vCf), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &vCf), project_vertex(camera, &vDf), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &vDf), project_vertex(camera, &vAf), Color::BLACK);

    draw_line(canvas, project_vertex(camera, &vAb), project_vertex(camera, &vBb), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &vBb), project_vertex(camera, &vCb), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &vCb), project_vertex(camera, &vDb), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &vDb), project_vertex(camera, &vAb), Color::BLACK);

    draw_line(canvas, project_vertex(camera, &vAf), project_vertex(camera, &vAb), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &vBf), project_vertex(camera, &vBb), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &vCf), project_vertex(camera, &vCb), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &vDf), project_vertex(camera, &vDb), Color::BLACK);
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = WyRand::new();
    let mut camera = Camera::new(Some(Vec3::new(0.0, 0.0, -10.0)), None, Some(90u8), (800, 600));
    // sleep(Duration::new(5, 0));
    'running: loop {
        // put_color(&mut canvas, Point::new(rng.generate_range::<u32>(1, 800) as i32, rng.generate_range::<u32>(1, 600) as i32), Color::BLACK);
        // draw_line(&mut canvas, Point::new(400, 300), Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32), Color::BLACK);
        // draw_triangle(&mut canvas,
        //               Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
        //               Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
        //               Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
        //               Color::BLACK);
        // draw_filled_triangle(&mut canvas,
        //               Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
        //               Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
        //               Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
        //               Color::BLACK, Color::GREEN);
        // draw_shaded_triangle(&mut canvas,
        //                      XYH::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32, rand_percent(&mut rng)),
        //                      XYH::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32, rand_percent(&mut rng)),
        //                      XYH::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32, rand_percent(&mut rng)),
        //                 Color::GREEN);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    camera.move_(Vec3::new(0.0, 0.0, 0.1));
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    camera.move_(Vec3::new(-0.1, 0.0, 0.0));
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    camera.move_(Vec3::new(0.0, 0.0, -0.1));
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    camera.move_(Vec3::new(0.1, 0.0, 0.0));
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        draw_cube(&mut canvas, 1, &camera);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 75));
    }
}