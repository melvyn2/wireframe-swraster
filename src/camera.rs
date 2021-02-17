use std::cmp::Ordering;

use crate::math::*;

pub struct Camera {
    pub pos: Vec3,
    pub rot: Quat,
    res: (u32, u32),
    pub fov: u8,
    pub viewport: Vec4
}

impl Camera {
    pub fn new(pos: Option<Vec3>, rot: Option<Quat>, fov: Option<u8>, res: (u32, u32)) -> Camera {
        let pos = pos.unwrap_or_default();
        let rot = rot.unwrap_or_default();
        let fov = fov.unwrap_or(90);
        let viewport = viewport(fov, res, None);
        Camera{pos, rot, res, fov, viewport}
    }
    pub fn local_move(&mut self, offset: Vec3) {
        self.pos += self.rot.conjugate() * offset;
    }
    pub fn look(&mut self, offset: Quat) {
        self.rot *= offset;
    }
    pub fn change_fov(&mut self, fov: u8) {
        if fov <= 0 || fov >= 180 {
            return;
        }
        self.viewport = viewport(fov, self.res, Some(self.viewport.z));
        self.fov = fov;
    }
    pub fn change_res(&mut self, res: (u32, u32)) {
        self.viewport = viewport(self.fov, res, Some(self.viewport.z));
        self.res = res;
    }
}

fn viewport(fov: u8, res: (u32, u32), d: Option<FP>) -> Vec4 {
    // Create a viewport plane d away from the camera with a matching FOV and aspect ratio
    // The longer side according to the ratio will have the full FOV, while the other will be scaled accordingly

    let side_bc = d.unwrap_or(1.0);
    let angle_c = (fov as FP)/180.0 * crate::math::FRAC_PI_2;
    let angle_a = crate::math::FRAC_PI_2 - angle_c;
    let side_ab = side_bc * (angle_c.sin() / angle_a.sin());

    // Resize shorter size to fit ratio
    let ratio = res.0 as FP/res.1 as FP;
    match ratio.partial_cmp(&(1 as FP)).unwrap() {
        Ordering::Equal => Vec4::new(side_ab * 2 as FP, side_ab * 2 as FP, side_bc, res.0 as FP /(side_ab * 2 as FP)),
        Ordering::Less => Vec4::new(side_ab * 2 as FP * ratio, side_ab * 2 as FP, side_bc, res.1 as FP /(side_ab * 2 as FP)),
        Ordering::Greater => Vec4::new(side_ab * 2 as FP, side_ab * 2 as FP * (1 as FP/ratio), side_bc, res.0 as FP /(side_ab * 2 as FP))
    }
}