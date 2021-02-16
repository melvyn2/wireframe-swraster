#![feature(destructuring_assignment)]
#![feature(option_expect_none)]

use std::time::Duration;
use std::collections::HashMap;
use std::rc::Rc;

extern crate nanorand;
use nanorand::{RNG, WyRand};

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{WindowCanvas};

mod math;
use math::*;

mod camera;
use camera::*;

mod flatshapes;
use flatshapes::*;

struct Mesh {
    verts: Vec<Vec3>,
    tris: Vec<(Vec3, Vec3, Vec3)>
}
struct Object {
    pos: Vec3,
    rot: Quat,
    mesh: Rc<Mesh>
}

fn cube(scale: i32) -> Object {
    let pos = Vec3::zero();
    let rot = Quat::default();
    let verts: Vec<Vec3> = vec![Vec3::new(-scale as FP, -scale as FP, 0.0),
                     Vec3::new(scale as FP, -scale as FP, 0.0),
                     Vec3::new(scale as FP, scale as FP, 0.0),
                     Vec3::new(-scale as FP, scale as FP, 0.0),
                     Vec3::new(-scale as FP, -scale as FP, scale as FP),
                     Vec3::new(scale as FP, -scale as FP, scale as FP),
                     Vec3::new(scale as FP, scale as FP, scale as FP),
                     Vec3::new(-scale as FP, scale as FP, scale as FP)];
    let tris = vec![(verts[0], verts[1], verts[2]),
                             (verts[0], verts[2], verts[3]),
                             (verts[4], verts[0], verts[3]),
                             (verts[4], verts[3], verts[7]),
                             (verts[5], verts[4], verts[7]),
                             (verts[5], verts[7], verts[6]),
                             (verts[1], verts[5], verts[6]),
                             (verts[1], verts[6], verts[2]),
                             (verts[4], verts[5], verts[1]),
                             (verts[4], verts[1], verts[0]),
                             (verts[2], verts[6], verts[7]),
                             (verts[2], verts[7], verts[3])];
    let mesh = Mesh{verts, tris};
    Object{pos, rot, mesh: Rc::new(mesh)}
}

struct Scene {
    camera: Camera,
    objects: Vec<Object>
}

fn rand_percent(rng: &mut WyRand) -> FP {
    rng.generate_range::<u16>(0, 10000) as FP / 10000.0
}

fn project_vertex(camera: &Camera, point: &Vec3) -> Point {
    // Switch to camera space
    let point_local = *point - camera.pos;
    // TODO apply camera rotation

    if point_local.z <= camera.viewport.z {
        return Point::new(0, 0)
    }
    // camera.viewport.z / point_local.z  projects to viewport
    let vp_res = point_local.xy() * camera.viewport.z / point_local.z;
    // camera.viewport.w scales to canvas
    let res = (vp_res + (camera.viewport.xy() / 2.0)) * camera.viewport.w;
    Point::new(res.x as i32, res.y as i32)
}

fn render_object(canvas: &mut WindowCanvas, camera: &Camera, obj: &Object) {
    let mut vmap: HashMap<u64, Point> = HashMap::new();
    for v in &obj.mesh.verts {
        vmap.insert(vec3_hash(v), project_vertex(camera, &(*v + obj.pos))).expect_none("Dupe vertex key");
    }
    for (t0, t1, t2) in &obj.mesh.tris {
        draw_triangle(canvas, *vmap.get(&vec3_hash(t0)).unwrap(),
                      *vmap.get(&vec3_hash(t1)).unwrap(),
                      *vmap.get(&vec3_hash(t2)).unwrap(), Color::BLACK);
    }
}

fn draw_cube(canvas: &mut WindowCanvas, scale: i32, camera: &Camera) {
    let fv_a = Vec3::new(-scale as FP, -scale as FP, 0.0);
    let fv_b = Vec3::new(scale as FP, -scale as FP, 0.0);
    let fv_c = Vec3::new(scale as FP, scale as FP, 0.0);
    let fv_d = Vec3::new(-scale as FP, scale as FP, 0.0);

    let bv_a = Vec3::new(-scale as FP, -scale as FP, scale as FP);
    let bc_b = Vec3::new(scale as FP, -scale as FP, scale as FP);
    let bv_d = Vec3::new(scale as FP, scale as FP, scale as FP);
    let bv_e = Vec3::new(-scale as FP, scale as FP, scale as FP);

    draw_line(canvas, project_vertex(camera, &fv_a), project_vertex(camera, &fv_b), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &fv_b), project_vertex(camera, &fv_c), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &fv_c), project_vertex(camera, &fv_d), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &fv_d), project_vertex(camera, &fv_a), Color::BLACK);

    draw_line(canvas, project_vertex(camera, &bv_a), project_vertex(camera, &bc_b), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &bc_b), project_vertex(camera, &bv_d), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &bv_d), project_vertex(camera, &bv_e), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &bv_e), project_vertex(camera, &bv_a), Color::BLACK);

    draw_line(canvas, project_vertex(camera, &fv_a), project_vertex(camera, &bv_a), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &fv_b), project_vertex(camera, &bc_b), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &fv_c), project_vertex(camera, &bv_d), Color::BLACK);
    draw_line(canvas, project_vertex(camera, &fv_d), project_vertex(camera, &bv_e), Color::BLACK);
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut rng = WyRand::new();
    let mut camera = Camera::new(Some(Vec3::new(0.0, 0.0, -10.0)), None, Some(90u8), (800, 600));
    let obj = cube(2);
    render_object(&mut canvas, &camera, &obj);
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
        render_object(&mut canvas, &camera, &obj);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 75));
    }
}