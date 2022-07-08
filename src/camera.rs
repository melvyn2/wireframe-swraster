use std::cmp::Ordering;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::math::*;

pub struct Camera {
    pub pos: Vec3,
    pub rot: Quat,
    res: (u32, u32),
    pub fov: u8,
    pub viewport: Vec4,
}

impl Camera {
    pub fn new(pos: Option<Vec3>, rot: Option<Quat>, fov: Option<u8>, res: (u32, u32)) -> Camera {
        let pos = pos.unwrap_or_default();
        let rot = rot.unwrap_or_default();
        let fov = fov.unwrap_or(90);
        let viewport = viewport(fov, res, None);
        Camera {
            pos,
            rot,
            res,
            fov,
            viewport,
        }
    }
    pub fn local_move(&mut self, offset: Vec3) {
        self.pos += self.rot.conjugate() * offset;
    }
    pub fn look(&mut self, offset: Quat) {
        self.rot *= offset;
        self.rot = self.rot.normalize();
    }
    pub fn change_fov(&mut self, fov: u8) {
        if fov == 0 || fov >= 180 {
            return;
        }
        self.viewport = viewport(fov, self.res, Some(self.viewport.z));
        self.fov = fov;
    }
    pub fn change_res(&mut self, res: (u32, u32)) {
        self.viewport = viewport(self.fov, res, Some(self.viewport.z));
        self.res = res;
    }
    pub fn process_inputs(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    std::process::exit(0); // Would be better to handle in main()
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    self.local_move(Vec3::new(0.0, 0.0, 0.1));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    self.local_move(Vec3::new(-0.1, 0.0, 0.0));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    self.local_move(Vec3::new(0.0, 0.0, -0.1));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.local_move(Vec3::new(0.1, 0.0, 0.0));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    self.look(Quat::from_rotation_x(-0.01));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    self.look(Quat::from_rotation_x(0.01));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    self.look(Quat::from_rotation_y(0.01));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self.look(Quat::from_rotation_y(-0.01));
                }
                Event::MouseWheel { y, .. } => {
                    self.change_fov((self.fov as i32 + y) as u8);
                }
                Event::Window {
                    win_event: WindowEvent::SizeChanged(x, y),
                    ..
                } => {
                    self.change_res((x as u32, y as u32));
                }
                // Mouse movement disabled because an mouse unlock is needed and the camera rolls
                // Event::MouseMotion { xrel, yrel, .. } => self.look(
                //     Quat::from_rotation_y(xrel as FP / 100.0)
                //         * Quat::from_rotation_x(yrel as FP / 100.0),
                // ),
                _ => {}
            }
        }
    }
}

fn viewport(fov: u8, res: (u32, u32), d: Option<FP>) -> Vec4 {
    // Create a viewport plane d away from the camera with a matching FOV and aspect ratio
    // The longer side according to the ratio will have the full FOV, while the other will be scaled accordingly

    let side_bc = d.unwrap_or(1.0);
    let angle_c = (fov as FP) / 180.0 * crate::math::FRAC_PI_2;
    let angle_a = crate::math::FRAC_PI_2 - angle_c;
    let side_ab = side_bc * (angle_c.sin() / angle_a.sin());

    // Resize shorter size to fit ratio
    let ratio = res.0 as FP / res.1 as FP;
    match ratio.partial_cmp(&(1.0)).unwrap() {
        Ordering::Equal => Vec4::new(
            side_ab * 2.0,
            side_ab * 2.0,
            side_bc,
            res.0 as FP / (side_ab * 2.0),
        ),
        Ordering::Less => Vec4::new(
            side_ab * 2.0 * ratio,
            side_ab * 2.0,
            side_bc,
            res.1 as FP / (side_ab * 2.0),
        ),
        Ordering::Greater => Vec4::new(
            side_ab * 2.0,
            side_ab * 2.0 * ratio.recip(),
            side_bc,
            res.0 as FP / (side_ab * 2.0),
        ),
    }
}
