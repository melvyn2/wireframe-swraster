use std::ops;

use glam::{DVec2, DVec3, DVec4, DQuat};
pub use glam::{IVec2, IVec3};
pub use glam::{Vec3Swizzles, Vec4Swizzles};
use std::mem::transmute;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Annoying way to change float precision easily
pub type FP = f64;

pub type Vec2 = DVec2;
pub type Vec3 = DVec3;
pub type Vec4 = DVec4;

pub type Quat = DQuat;

pub struct XYH {
    pub x: i32,
    pub y: i32,
    pub h: FP
}

impl XYH {
    pub fn new(x: i32, y: i32, h: FP) -> XYH {
        XYH{x, y, h}
    }
}

macro_rules! assert_eq_fp {
    ($lhs:expr, $rhs:expr) => {
        assert!(($lhs - $rhs).abs() < FP::EPSILON);
    };
    ($lhs:expr, $rhs:expr, $tolerance:expr) => {
        assert!(($lhs - $rhs).abs() < $tolerance);
    };
}

pub fn lerp(x1: i32, y1: FP, x2: i32, y2: FP) -> Vec<FP> {
    if x1 == x2 {
        return vec![y1];
    }
    let fx1 = x1 as FP;
    let fx2 = x2 as FP;
    let m = (y2 - y1) / (fx2 - fx1);
    let b = y1 - (m * fx1);
    assert_eq_fp!(y1, (m * fx1) + b, 0.001);
    assert_eq_fp!(y2, (m * fx2) + b, 0.001);
    ops::RangeInclusive::new(x1, x2).map(|x| (x as FP * m) + b).collect()
}

pub fn vec3_hash(v: &Vec3) -> u64 {
let mut h = DefaultHasher::new();
    unsafe {transmute::<f64, i64>(v.x)}.hash(&mut h);
    unsafe {transmute::<f64, i64>(v.y)}.hash(&mut h);
    unsafe {transmute::<f64, i64>(v.z)}.hash(&mut h);
    h.finish()
}