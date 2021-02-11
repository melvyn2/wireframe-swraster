use std::cmp::Ordering;

use crate::math::*;

pub struct Camera {
    pub pos: Vec3,
    rot: Quat,
    pub viewport: Vec4
}

impl Camera {
    pub fn new(pos: Option<Vec3>, rot: Option<Quat>, fov: Option<u8>, res: (u32, u32)) -> Camera {
        let pos = pos.unwrap_or_default();
        let rot = rot.unwrap_or_default();
        let viewport = viewport(fov.unwrap_or(90), res, None);
        Camera{pos, rot, viewport}
    }
    pub fn move_(&mut self, offset: Vec3) {
        self.pos += offset;
    }
}

fn viewport(fov: u8, res: (u32, u32), d: Option<FP>) -> Vec4 {
    // Create a viewport plane d away from the camera with a matching FOV and aspect ratio
    // The longer side (according to the ratio will have the full FOV, while the other will be scaled accordingly
    /*
        Top-down view
                 B
      A ____________________ D
        \        |
         \       |
          \      |
           \     |
            \    |
             \   |
              \  |
               \ |
                \|
                 C
        _BC = d (distance from viewport)
        ∠b = 90°
        ∠c = (fov/2)°
        ∠a = 90 - (fov/2)°

        Solving for _AD (viewport width)
        _AD = _AB * 2
        _AB/sin ∠c = _BC/sin ∠a
        _AB = (_BC/sin ∠a) * sin ∠c
     */
    let side_bc = d.unwrap_or(1 as FP);
    let angle_c = (fov as FP)/(2 as FP);
    let angle_a = (90 as FP) - angle_c;
    let side_ab = (side_bc / angle_a.sin()) * angle_c.sin();

    // Resize shorter size to fit ratio
    let ratio = res.0 as FP/res.1 as FP;
    match ratio.partial_cmp(&(1 as FP)).unwrap() {
        Ordering::Equal => Vec4::new(side_ab * 2 as FP, side_ab * 2 as FP, side_bc, res.0 as FP /(side_ab * 2 as FP)),
        Ordering::Less => Vec4::new(side_ab * 2 as FP * ratio, side_ab * 2 as FP, side_bc, res.1 as FP /(side_ab * 2 as FP)),
        Ordering::Greater => Vec4::new(side_ab * 2 as FP, side_ab * 2 as FP * (1 as FP/ratio), side_bc, res.0 as FP /(side_ab * 2 as FP))
    }
}