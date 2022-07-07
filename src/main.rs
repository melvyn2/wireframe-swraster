#![feature(test)]
#![allow(dead_code)]

use std::rc::Rc;
use std::time;

use nanorand::{Rng, WyRand};

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

mod camera;
mod flatshapes;
mod math;
mod meshes;
mod object;

use camera::*;
use math::*;
use meshes::*;
use object::*;

#[cfg(test)]
mod bench;

struct Scene {
    camera: Camera,
    objects: Vec<Object>,
}

fn rand_percent(rng: &mut WyRand) -> FP {
    rng.generate_range(0_u16..=10000) as FP / 10000.0
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
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("graphics demo", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    // sdl_context.mouse().set_relative_mouse_mode(true);
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut rng = WyRand::new();
    // let mut imp_mesh = import_mesh(Path::new("src/teapot.glb"));
    // imp_mesh.truncate(imp_mesh.len() - (imp_mesh.len() % 3));
    let mut camera = Camera::new(
        Some(Vec3::new(0.0, 0.0, -10.0)),
        None,
        Some(90u8),
        (800, 600),
    );
    let obj = Object {
        pos: Vec3::new(0.0, 3.0, 0.0),
        rot: Quat::default(),
        scale: 4.0,
        mesh: Rc::new(teapot()),
    };
    obj.render(&mut canvas, &camera);
    'running: loop {
        let fr_start = time::Instant::now();
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
        //                 Color::from((rng.generate_range::<u8>(0, 255), rng.generate_range::<u8>(0, 255), rng.generate_range::<u8>(0, 255))));
        // draw_multishade_triangle(&mut canvas, Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
        //                                        Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
        //                                        Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32), Color::RED, Color::GREEN, Color::BLUE);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    camera.local_move(Vec3::new(0.0, 0.0, 0.1));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    camera.local_move(Vec3::new(-0.1, 0.0, 0.0));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    camera.local_move(Vec3::new(0.0, 0.0, -0.1));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    camera.local_move(Vec3::new(0.1, 0.0, 0.0));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    camera.look(Quat::from_rotation_x(-0.01));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    camera.look(Quat::from_rotation_x(0.01));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    camera.look(Quat::from_rotation_y(0.01));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    camera.look(Quat::from_rotation_y(-0.01));
                }
                Event::MouseWheel { y, .. } => {
                    camera.change_fov((camera.fov as i32 + y) as u8);
                }
                Event::Window {
                    win_event: WindowEvent::SizeChanged(x, y),
                    ..
                } => {
                    camera.change_res((x as u32, y as u32));
                }
                // Event::MouseMotion { xrel, yrel, .. } => camera.look(
                //     Quat::from_rotation_x(xrel as FP / 10.0)
                //         * Quat::from_rotation_y(yrel as FP / 10.0),
                // ),
                _ => {}
            }
        }
        // camera.rot.normalize();
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        obj.render(&mut canvas, &camera);
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
