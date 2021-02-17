#![feature(destructuring_assignment)]
#![feature(option_expect_none)]
#![feature(test)]
#![allow(dead_code)]

use std::rc::Rc;
use std::time::Duration;

use nanorand::{WyRand, RNG};

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

mod camera;
mod flatshapes;
mod math;

use camera::*;
use flatshapes::*;
use math::*;

#[cfg(test)]
mod bench;

struct Mesh {
    tris: Vec<Vec3>,
}

struct Object {
    pos: Vec3,
    rot: Quat,
    mesh: Rc<Mesh>,
}

fn cube(scale: i32) -> Object {
    let pos = Vec3::zero();
    let rot = Quat::from_rotation_y(0.2);
    let scale = scale as FP;
    let verts: Vec<Vec3> = vec![
        Vec3::new(-scale, -scale, -scale),
        Vec3::new(scale, -scale, -scale),
        Vec3::new(scale, scale, -scale),
        Vec3::new(-scale, scale, -scale),
        Vec3::new(-scale, -scale, scale),
        Vec3::new(scale, -scale, scale),
        Vec3::new(scale, scale, scale),
        Vec3::new(-scale, scale, scale),
    ];
    let tris = vec![
        verts[0], verts[1], verts[2], verts[0], verts[2], verts[3], verts[4], verts[0], verts[3],
        verts[4], verts[3], verts[7], verts[5], verts[4], verts[7], verts[5], verts[7], verts[6],
        verts[1], verts[5], verts[6], verts[1], verts[6], verts[2], verts[4], verts[5], verts[1],
        verts[4], verts[1], verts[0], verts[2], verts[6], verts[7], verts[2], verts[7], verts[3],
    ];
    let mesh = Mesh { tris };
    Object {
        pos,
        rot,
        mesh: Rc::new(mesh),
    }
}

struct Scene {
    camera: Camera,
    objects: Vec<Object>,
}

fn rand_percent(rng: &mut WyRand) -> FP {
    rng.generate_range::<u16>(0, 10000) as FP / 10000.0
}

fn project_vertex(camera: &Camera, point: &Vec3) -> Point {
    // Switch to camera space
    let point_local = camera.rot * (*point - camera.pos);
    // TODO apply camera rotation

    if point_local.z <= camera.viewport.z {
        return Point::new(0, 0);
    }
    // camera.viewport.z / point_local.z  projects to viewport
    let vp_res = point_local.xy() * camera.viewport.z / point_local.z;
    // camera.viewport.w scales to canvas
    let res = (vp_res + (camera.viewport.xy() / 2.0)) * camera.viewport.w;
    Point::new(res.x as i32, res.y as i32)
}

fn render_object(canvas: &mut WindowCanvas, camera: &Camera, obj: &Object) {
    assert_eq!(obj.mesh.tris.len() % 3, 0);
    for idx in (0..obj.mesh.tris.len()).step_by(3) {
        draw_triangle(
            canvas,
            project_vertex(camera, &obj.mesh.tris[idx]),
            project_vertex(camera, &obj.mesh.tris[idx + 1]),
            project_vertex(camera, &obj.mesh.tris[idx + 2]),
            Color::BLACK,
        );
    }
}

fn draw_cube(canvas: &mut WindowCanvas, scale: i32, camera: &Camera) {
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

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("graphics demo", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut rng = WyRand::new();
    let mut camera = Camera::new(
        Some(Vec3::new(0.0, 0.0, -10.0)),
        None,
        Some(90u8),
        (800, 600),
    );
    let obj = cube(2);
    render_object(&mut canvas, &camera, &obj);
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
                _ => {}
            }
        }
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        render_object(&mut canvas, &camera, &obj);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 75));
    }
}
