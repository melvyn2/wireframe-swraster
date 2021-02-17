extern crate test;
use test::Bencher;

use crate::*;

fn get_test_canvas() -> WindowCanvas {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("graphics benchmark", 800, 600)
        .position_centered()
        .hidden()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();
    canvas
}

#[bench]
fn lerp(b: &mut Bencher) {
    let p0 = (1_000_000, 0.0);
    let p1 = (4, 1_337.0);
    b.iter(|| crate::lerp(p0.0, p0.1, p1.0, p1.1));
    dbg!(crate::lerp(p0.0, p0.1, p1.0, p1.1));
}

#[bench]
fn pixel(b: &mut Bencher) {
    let mut canvas = get_test_canvas();
    b.iter(|| put_color(&mut canvas, Point::new(100, 100), Color::BLACK))
}

#[bench]
fn line(b: &mut Bencher) {
    let mut canvas = get_test_canvas();
    b.iter(|| {
        draw_line(
            &mut canvas,
            Point::new(100, 100),
            Point::new(200, 200),
            Color::BLACK,
        )
    });
}

#[bench]
fn wf_tri(b: &mut Bencher) {
    let mut canvas = get_test_canvas();
    b.iter(|| {
        draw_triangle(
            &mut canvas,
            Point::new(100, 100),
            Point::new(200, 120),
            Point::new(120, 200),
            Color::BLACK,
        )
    });
}

#[bench]
fn filled_tri(b: &mut Bencher) {
    let mut canvas = get_test_canvas();
    b.iter(|| {
        draw_filled_triangle(
            &mut canvas,
            Point::new(100, 100),
            Point::new(200, 120),
            Point::new(120, 200),
            Color::BLACK,
            Color::GREEN,
        )
    });
}

#[bench]
fn shaded_tri(b: &mut Bencher) {
    let mut canvas = get_test_canvas();
    b.iter(|| {
        draw_shaded_triangle(
            &mut canvas,
            XYH::new(100, 100, 1.0),
            XYH::new(200, 120, 0.7),
            XYH::new(120, 200, 0.0),
            Color::GREEN,
        )
    });
}

#[bench]
fn multishade_tri(b: &mut Bencher) {
    let mut canvas = get_test_canvas();
    b.iter(|| {
        draw_multishade_triangle(
            &mut canvas,
            Point::new(100, 100),
            Point::new(200, 120),
            Point::new(120, 200),
            Color::RED,
            Color::GREEN,
            Color::BLUE,
        )
    });
}

#[bench]
fn project(b: &mut Bencher) {
    let camera = Camera::new(None, None, None, (800, 600));
    let v = Vec3::new(1.0, 700.0, 46.0);
    b.iter(|| project_vertex(&camera, &v));
}

#[bench]
fn cube(b: &mut Bencher) {
    let mut canvas = get_test_canvas();
    let camera = Camera::new(None, None, None, (800, 600));
    let cube = Object {
        pos: Vec3::zero(),
        rot: Quat::default(),
        scale: 4.0,
        mesh: Rc::new(crate::cube(1)),
    };
    b.iter(|| render_object(&mut canvas, &camera, &cube));
}
