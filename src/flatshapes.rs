use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use crate::math::*;

pub fn put_color(canvas: &mut WindowCanvas, p: Point, c: Color) {
    canvas.set_draw_color(c);
    canvas.draw_point(p).unwrap();
}

pub fn draw_line(canvas: &mut WindowCanvas, p0: Point, p1: Point, c: Color) {
    if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
        let (p0, p1) = match p0.x > p1.x {
            true => (p1, p0),
            false => (p0, p1),
        };
        let ys = lerp(p0.x, p0.y as FP, p1.x, p1.y as FP);
        for point in ys.iter().zip(p0.x..=p1.x) {
            put_color(canvas, Point::new(point.1, *point.0 as i32), c);
        }
    } else {
        let (p0, p1) = match p0.y > p1.y {
            true => (p1, p0),
            false => (p0, p1),
        };
        let xs = lerp(p0.y, p0.x as FP, p1.y, p1.x as FP);
        for point in xs.iter().zip(p0.y..=p1.y) {
            put_color(canvas, Point::new(*point.0 as i32, point.1), c);
        }
    }
}

pub fn draw_triangle(canvas: &mut WindowCanvas, p0: Point, p1: Point, p2: Point, c: Color) {
    draw_line(canvas, p0, p1, c);
    draw_line(canvas, p0, p2, c);
    draw_line(canvas, p1, p2, c);
}

pub fn draw_filled_triangle(
    canvas: &mut WindowCanvas,
    p0: Point,
    p1: Point,
    p2: Point,
    outline_color: Color,
    fill_color: Color,
) {
    // Sort points so that p0.y < p1.y < p2.y
    let (p0, p1) = match p1.y < p0.y {
        true => (p1, p0),
        false => (p0, p1),
    };
    let (p0, p2) = match p2.y < p0.y {
        true => (p2, p0),
        false => (p0, p2),
    };
    let (p1, p2) = match p2.y < p1.y {
        true => (p2, p1),
        false => (p1, p2),
    };
    assert!(p0.y <= p1.y && p1.y <= p2.y);

    // Create vecs of x coords of lines 01, 12, 02
    // 02 is long, 01 + 12 are shorts
    let mut x01_12 = lerp(p0.y, p0.x as FP, p1.y, p1.x as FP);
    // End of 01 and start of 02 are same point
    x01_12.pop();
    // Combine 2 short sides
    x01_12.append(&mut lerp(p1.y, p1.x as FP, p2.y, p2.x as FP));
    let x02 = lerp(p0.y, p0.x as FP, p2.y, p2.x as FP);
    // Make immutable
    let x01_12 = x01_12;

    // Find which side is left/right
    let m = x02.len() / 2;
    let (x_left, x_right) = match x02[m] < x01_12[m] {
        true => (x02, x01_12),
        false => (x01_12, x02),
    };

    for y in p0.y..p2.y {
        let idx = (y - p0.y) as usize;
        let x_l = x_left[idx] as i32;
        let x_r = x_right[idx] as i32;
        for x in x_l..x_r {
            put_color(canvas, Point::new(x, y), fill_color);
        }
    }
    draw_triangle(canvas, p0, p1, p2, outline_color);
}

pub fn draw_shaded_triangle(
    canvas: &mut WindowCanvas,
    p0: XYH,
    p1: XYH,
    p2: XYH,
    colorbase: Color,
) {
    let (p0, p1) = match p1.y < p0.y {
        true => (p1, p0),
        false => (p0, p1),
    };
    let (p0, p2) = match p2.y < p0.y {
        true => (p2, p0),
        false => (p0, p2),
    };
    let (p1, p2) = match p2.y < p1.y {
        true => (p2, p1),
        false => (p1, p2),
    };
    assert!(p0.y <= p1.y && p1.y <= p2.y);

    // Create vecs of x coords of lines 01, 12, 02
    // 02 is long, 01 + 12 are shorts
    let mut x01_12 = lerp(p0.y, p0.x as FP, p1.y, p1.x as FP);
    // End of 01 and start of 02 are same point
    x01_12.pop();
    // Combine 2 short sides
    x01_12.append(&mut lerp(p1.y, p1.x as FP, p2.y, p2.x as FP));
    // Make immutable
    let x01_12 = x01_12;
    let x02 = lerp(p0.y, p0.x as FP, p2.y, p2.x as FP);

    // Do the same for H
    let mut h01_12 = lerp(p0.y, p0.h, p1.y, p1.h);
    h01_12.pop();
    h01_12.append(&mut lerp(p1.y, p1.h, p2.y, p2.h));
    let h01_12 = h01_12;
    let h02 = lerp(p0.y, p0.h, p2.y, p2.h as FP);

    // Find which side is left/right
    let m = x02.len() / 2;
    let (x_left, h_left, x_right, h_right) = match x02[m] < x01_12[m] {
        true => (x02, h02, x01_12, h01_12),
        false => (x01_12, h01_12, x02, h02),
    };

    // Draw triangle
    for y in p0.y..p2.y {
        let idx = (y - p0.y) as usize;
        let x_l = x_left[idx] as i32;
        let x_r = x_right[idx] as i32;
        let h_seg = lerp(x_l, h_left[idx], x_r, h_right[idx]);
        for x in x_l..x_r {
            let h = h_seg[(x - x_l) as usize];
            let r = (colorbase.r as FP * h) as u8;
            let g = (colorbase.g as FP * h) as u8;
            let b = (colorbase.b as FP * h) as u8;
            put_color(canvas, Point::new(x, y), Color::from((r, g, b)));
        }
    }
    // draw_triangle(canvas, Point::new(p0.x, p0.y), Point::new(p1.x, p1.y), Point::new(p2.x, p2.y), colorbase);
}

pub fn draw_multishade_triangle(
    canvas: &mut WindowCanvas,
    p0: Point,
    p1: Point,
    p2: Point,
    c0: Color,
    c1: Color,
    c2: Color,
) {
    // Probably inefficient in so many ways
    let c0 = Vec3::new(c0.r as FP, c0.g as FP, c0.b as FP);
    let c1 = Vec3::new(c1.r as FP, c1.g as FP, c1.b as FP);
    let c2 = Vec3::new(c2.r as FP, c2.g as FP, c2.b as FP);

    let (p0, c0, p1, c1) = match p1.y < p0.y {
        true => (p1, c1, p0, c0),
        false => (p0, c0, p1, c1),
    };
    let (p0, c0, p2, c2) = match p2.y < p0.y {
        true => (p2, c2, p0, c0),
        false => (p0, c0, p2, c2),
    };
    let (p1, c1, p2, c2) = match p2.y < p1.y {
        true => (p2, c2, p1, c1),
        false => (p1, c1, p2, c2),
    };

    let mut x01_12 = lerp(p0.y, p0.x as FP, p1.y, p1.x as FP);
    x01_12.pop();
    x01_12.append(&mut lerp(p1.y, p1.x as FP, p2.y, p2.x as FP));
    let x02 = lerp(p0.y, p0.x as FP, p2.y, p2.x as FP);
    let x01_12 = x01_12;

    let mut c01_12 = (0..p1.y - p0.y)
        .map(|x| c0.lerp(c1, x as FP / (p1.y - p0.y) as FP))
        .collect::<Vec<Vec3>>();
    c01_12.append(
        &mut (0..=(p2.y - p1.y))
            .map(|x| c1.lerp(c2, x as FP / (p2.y - p1.y) as FP))
            .collect::<Vec<Vec3>>(),
    );
    let c02 = (0..=(p2.y - p0.y))
        .map(|x| c0.lerp(c2, x as FP / (p2.y - p0.y) as FP))
        .collect::<Vec<Vec3>>();
    let c01_12 = c01_12;

    let m = x02.len() / 2;
    let (x_left, c_left, x_right, c_right) = match x02[m] < x01_12[m] {
        true => (x02, c02, x01_12, c01_12),
        false => (x01_12, c01_12, x02, c02),
    };

    for y in p0.y..p2.y {
        let idx = (y - p0.y) as usize;
        let x_l = x_left[idx] as i32;
        let c_l = c_left[idx];
        let x_r = x_right[idx] as i32;
        let c_r = c_right[idx];
        for x in x_l..x_r {
            let c_vec = c_l.lerp(c_r, (x - x_l) as FP / (x_r - x_l) as FP);
            let color = Color::from((c_vec.x as u8, c_vec.y as u8, c_vec.z as u8));
            put_color(canvas, Point::new(x, y), color);
        }
    }
}
