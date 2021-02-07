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

fn lerp(x1: i32, y1: f64, x2: i32, y2: f64) -> Vec<f64> {
    if x1 == x2 {
        return vec![y1];
    }
    let fx1: f64 = x1 as f64;
    let fx2: f64 = x2 as f64;
    let m: f64 = (y2 - y1) / (fx2 - fx1);
    let b: f64 = y1 - (m * fx1);
    assert_eq_f64!(y1, (m * fx1) + b, 0.00001);
    assert_eq_f64!(y2, (m * fx2) + b, 0.00001);
    ops::RangeInclusive::new(x1, x2).map(|x| (x as f64 * m) + b).collect()
}

fn draw_line(canvas: &mut WindowCanvas, p0: Point, p1: Point, c: Color) {
    if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
        let (p0, p1) = match p0.x > p1.x {
            true => (p1, p0),
            false => (p0, p1)
        };
        let ys = lerp(p0.x, p0.y as f64, p1.x, p1.y as f64);
        for point in ys.iter().zip(p0.x..=p1.x) {
            put_color(canvas, Point::new(point.1, *point.0 as i32), c);
        }
    } else {
        let (p0, p1) = match p0.y > p1.y {
            true => (p1, p0),
            false => (p0, p1)
        };
        let xs = lerp(p0.y, p0.x as f64, p1.y, p1.x as f64);
        for point in xs.iter().zip(p0.y..=p1.y) {
            put_color(canvas, Point::new(*point.0 as i32, point.1), c);
        }
    }
}

fn draw_triangle(canvas: &mut WindowCanvas, p0: Point, p1: Point, p2: Point, c: Color) {
    draw_line(canvas, p0, p1, c);
    draw_line(canvas, p0, p2, c);
    draw_line(canvas, p1, p2, c);
}

fn draw_filled_triangle(canvas: &mut WindowCanvas, p0: Point, p1: Point, p2: Point, outline_color: Color, fill_color: Color) {
    // Sort points so that p0.y < p1.y < p2.y
    let (p0, p1) = match p1.y < p0.y {
        true => (p1, p0),
        false => (p0, p1)
    };
    let (p0, p2) = match p2.y < p0.y {
        true => (p2, p0),
        false => (p0, p2)
    };
    let (p1, p2) = match p2.y < p1.y {
        true => (p2, p1),
        false => (p1, p2)
    };
    assert!(p0.y <= p1.y && p1.y <= p2.y);

    // Create vecs of x coords of lines 01, 12, 02
    // 02 is long, 01 + 12 are shorts
    let mut x01 = lerp(p0.y, p0.x as f64, p1.y, p1.x as f64);
    let x12 = lerp(p1.y, p1.x as f64, p2.y, p2.x as f64);
    let x02 = lerp(p0.y, p0.x as f64, p2.y, p2.x as f64);
    // End of 01 and start of 02 are same point
    x01.pop();
    // Combine 2 short sides
    let mut x01_12 = x01.clone();
    x01_12.extend(x12);
    // Make immutable, costly so disabled
    // let x01_12 = &*x01_12.iter().collect::<Vec<_>>();


    let m = x02.len() / 2;
    let (x_left, x_right) = match x02[m] < x01_12[m] {
        true => (x02, x01_12),
        false => (x01_12, x02)
    };


    for y in p0.y..p2.y {
        for x in x_left[(y - p0.y) as usize] as i32..x_right[(y - p0.y) as usize] as i32 {
            put_color(canvas, Point::new(x, y), fill_color);
        }
    }
    draw_triangle(canvas, p0, p1, p2, outline_color);
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
    let mut rng = WyRand::new();
    'running: loop {
        // put_color(&mut canvas, Point::new(rng.generate_range::<u32>(1, 800) as i32, rng.generate_range::<u32>(1, 600) as i32), Color::BLACK);
        // draw_line(&mut canvas, Point::new(400, 300), Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32), Color::BLACK);
        // draw_triangle(&mut canvas,
        //               Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
        //               Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
        //               Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
        //               Color::BLACK);
        draw_filled_triangle(&mut canvas,
                      Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
                      Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
                      Point::new(rng.generate_range::<u32>(100, 700) as i32, rng.generate_range::<u32>(100, 500) as i32),
                      Color::BLACK, Color::GREEN);

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