#![feature(destructuring_assignment)]
extern crate nanorand;
extern crate sdl2;

use std::ops;
use std::time::Duration;

use nanorand::{RNG, WyRand};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

macro_rules! assert_eq_f64 {
    ($lhs:expr, $rhs:expr) => {
        assert!(($lhs - $rhs).abs() < f64::EPSILON);
    };
    ($lhs:expr, $rhs:expr, $tolerance:expr) => {
        assert!(($lhs - $rhs).abs() < $tolerance);
    };
}

fn put_color(canvas: &mut WindowCanvas, p: Point, c: Color) {
    canvas.set_draw_color(c);
    canvas.draw_point(p).unwrap();
}

fn lerp(x1: i32, y1: f64, x2: i32, y2: f64) -> Vec<(i32, f64)> {
    if x1 == x2 {
        return vec![(x1, y1)];
    }
    let fx1: f64 = x1 as f64;
    let fx2: f64 = x2 as f64;
    let m: f64 = (y2 - y1) / (fx2 - fx1);
    let b: f64 = y1 - (m * fx1);
    assert_eq_f64!(y1, (m * fx1) + b, 0.00001);
    assert_eq_f64!(y2, (m * fx2) + b, 0.00001);
    ops::RangeInclusive::new(x1, x2).map(|x| (x, (x as f64 * m) + b)).collect()
}

fn draw_line(canvas: &mut WindowCanvas, p0: Point, p1: Point, c: Color) {
    if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
        let (p0, p1) = match p0.x > p1.x {
            true => (p1, p0),
            false => (p0, p1)
        };
        let line = lerp(p0.x, p0.y as f64, p1.x, p1.y as f64);
        for point in line {
            put_color(canvas, Point::new(point.0, point.1 as i32), c);
        }
    } else {
        let (p0, p1) = match p0.y > p1.y {
            true => (p1, p0),
            false => (p0, p1)
        };
        let line = lerp(p0.y, p0.x as f64, p1.y, p1.x as f64);
        for point in line {
            put_color(canvas, Point::new(point.1 as i32, point.0), c);
        }
    }
}

fn draw_triangle(canvas: &mut WindowCanvas, p0: Point, p1: Point, p2: Point, c: Color) {
    draw_line(canvas, p0, p1, c);
    draw_line(canvas, p0, p2, c);
    draw_line(canvas, p1, p2, c);
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = WyRand::new();
    'running: loop {
        // put_color(&mut canvas, Point::new(rng.generate_range::<u32>(1, 800) as i32, rng.generate_range::<u32>(1, 600) as i32), Color::BLACK);
        // draw_line(&mut canvas, Point::new(400, 300), Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32), Color::BLACK);
        draw_triangle(&mut canvas,
                      Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
                      Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
                      Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
                      Color::BLACK);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}