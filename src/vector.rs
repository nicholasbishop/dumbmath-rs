// Copyright 2015 Nicholas Bishop
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ops::{Add, Div, Mul, Sub};

/// Vector with two f32 components
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32
}

impl Vec2f {
    /// Create a Vec2f from two components
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x: x,
                y: y }
    }

    pub fn cross(&self, other: Vec2f) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn lerp(&self, other: Vec2f, t: f32) -> Vec2f {
        *self * (1.0 - t) + other * t
    }

    pub fn dot(&self, other: Vec2f) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vec2f {
    type Output = Vec2f;
    fn add(self, v: Vec2f) -> Vec2f {
        Vec2f::new(self.x + v.x, self.y + v.y)
    }
}

impl Sub for Vec2f {
    type Output = Vec2f;
    fn sub(self, v: Vec2f) -> Vec2f {
        Vec2f::new(self.x - v.x, self.y - v.y)
    }
}

impl Mul<f32> for Vec2f {
    type Output = Vec2f;
    fn mul(self, s: f32) -> Vec2f {
        Vec2f::new(self.x * s, self.y * s)
    }
}

impl Div<f32> for Vec2f {
    type Output = Vec2f;
    fn div(self, s: f32) -> Vec2f {
        Vec2f::new(self.x / s, self.y / s)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line2f {
    pub points: (Vec2f, Vec2f)
}

impl Line2f {
    pub fn new(a: Vec2f, b: Vec2f) -> Line2f {
        Line2f { points: (a, b) }
    }

    pub fn closest_parametric_point(&self, point: Vec2f) -> f32 {
        let p0p = point - self.points.0;
        let p0p1 = self.points.1 - self.points.0;

        p0p.dot(p0p1) / p0p1.dot(p0p1)
    }
}

/// 2D Quadrilateral
/// 
/// ```text
///   p3____p2
///    |    |
///    |____|
///   p0    p1
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quad2f {
    pub points: (Vec2f, Vec2f, Vec2f, Vec2f)
}

/// Return type for Quad2f::iblerp
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBLerpResult {
    NoSolution,
    /// One solutions (as parametric coordinates)
    OneSolution(Vec2f),
    /// Two solutions (as parametric coordinates)
    TwoSolutions(Vec2f, Vec2f),
    ManySolutions
}

impl Quad2f {
    pub fn new(a: Vec2f, b: Vec2f, c: Vec2f, d: Vec2f) -> Quad2f {
        Quad2f {
            points: (a, b, c, d)
        }
    }

    /// Inverse bilinear interpolation
    ///
    /// Adapted from stackoverflow.com/questions/808441
    pub fn iblerp(&self, point: Vec2f) -> IBLerpResult {
        let p0mp = self.points.0 - point;
        let p1mp = self.points.1 - point;
        let p0mp3 = self.points.0 - self.points.3;
        let p1mp2 = self.points.1 - self.points.2;

        let a = p0mp.cross(p0mp3);
        let b0 = p0mp.cross(p1mp2);
        let b1 = p1mp.cross(p0mp3);
        let b = (b0 + b1) / 2.0;
        let c = p1mp.cross(p1mp2);

        let calc_st = |s| {
            let den = (1.0 - s) * p0mp3.x + s * p1mp2.x;
            let t = if den == 0.0 {
                // TODO(nicholasbishop): perhaps there's a more
                // efficient way to solve this, the SO post doesn't
                // seem to cover this case
                let rb = self.points.0.lerp(self.points.1, s);
                let rt = self.points.3.lerp(self.points.2, s);
                Line2f::new(rb, rt).closest_parametric_point(point)
            }
            else {
                ((1.0 - s) * (p0mp.x) + s * p1mp.x) / den
            };
            Vec2f::new(s, t)
        };

        let den = a - (2.0 * b) + c;
        if den == 0.0 {
            let m = a - c;
            if m == 0.0 {
                if a == 0.0 {
                    IBLerpResult::ManySolutions
                }
                else {
                    IBLerpResult::NoSolution
                }
            }
            else {
                IBLerpResult::OneSolution(calc_st(a / m))
            }
        }
        else {
            let left = a - b;
            let right = (b.powi(2) - a*c).sqrt();
            let s0 = (left + right) / den;
            let s1 = (left - right) / den;
            IBLerpResult::TwoSolutions(calc_st(s0), calc_st(s1))
        }
    }

    pub fn blerp(&self, u: f32, v: f32) -> Vec2f {
        let rb = self.points.0.lerp(self.points.1, u);
        let rt = self.points.3.lerp(self.points.2, v);
        rb.lerp(rt, v)
    }
}

#[test]
fn test_iblerp() {
    let q = Quad2f::new(vec2f(0, 0),
                        vec2f(4, 0),
                        vec2f(4, 4),
                        vec2f(0, 4));
    assert_eq!(q.iblerp(vec2f(2, 2)),
               IBLerpResult::OneSolution(vec2f(0.5, 0.5)));
}

/// Vector with three f32 components
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3f {
    /// Create a Vec3f from three components
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x: x,
                y: y,
                z: z }
    }

    /// Create a Vec3f with all components set to the same scalar
    pub fn from_scalar<T: CastF32>(s: T) -> Vec3f {
        let f = s.as_f32();
        Vec3f {
            x: f,
            y: f,
            z: f
        }
    }

    /// Squared length of the vector
    pub fn magnitude_squared(&self) -> f32 {
        dot3(*self, *self)
    }

    /// Length of the vector
    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    /// Create a normalized copy, or None if the magnitude is zero
    pub fn normalized(&self) -> Option<Vec3f> {
        let m = self.magnitude();
        if m == 0.0 {
            None
        }
        else {
            let f = 1.0 / m;
            Some((*self) * f)
        }
    }

    /// Projection of `self` into `v`, or None if the magnitude of
    /// `b` is zero.
    pub fn project_onto(&self, v: Vec3f) -> Option<Vec3f> {
        if v == ZERO_3F {
            None
        }
        else {
            Some(v * (dot3(*self, v) / dot3(v, v)))
        }
    }
}

/// Vec3f(0.0, 0.0, 0.0)
pub const ZERO_3F: Vec3f = Vec3f { x: 0.0, y: 0.0, z: 0.0 };

/// Convert a numeric type to an f32
pub trait CastF32 { fn as_f32(&self) -> f32; }

impl CastF32 for f32 { fn as_f32(&self) -> f32 { *self } }
impl CastF32 for f64 { fn as_f32(&self) -> f32 { *self as f32 } }

impl CastF32 for i8  { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for i16 { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for i32 { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for i64 { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for isize { fn as_f32(&self) -> f32 { *self as f32 } }

impl CastF32 for u8  { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for u16 { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for u32 { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for u64 { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for usize { fn as_f32(&self) -> f32 { *self as f32 } }

/// Create a Vec2f from x and y inputs
///
/// This is a convenience function that provides a little more
/// flexibility than Vec2f::new in that it will happily take numbers
/// that aren't f32 (including a mix of different types for each
/// component). Vec2f is such a common type that it seems reasonable
/// to provide a little extra ease of use.
pub fn vec2f<X: CastF32, Y: CastF32>(x: X, y: Y) -> Vec2f {
    Vec2f { x: x.as_f32(), y: y.as_f32() }
}

/// Create a Vec3f from x, y, and z inputs
///
/// This is a convenience function that provides a little more
/// flexibility than Vec3f::new in that it will happily take numbers
/// that aren't f32 (including a mix of different types for each
/// component). Vec3f is such a common type that it seems reasonable
/// to provide a little extra ease of use.
pub fn vec3f<X: CastF32, Y: CastF32, Z: CastF32>(x: X, y: Y, z: Z) -> Vec3f {
    Vec3f { x: x.as_f32(), y: y.as_f32(), z: z.as_f32() }
}

/// Inner product of two Vec3f inputs
pub fn dot3(a: Vec3f, b: Vec3f) -> f32 {
    let v = a * b;
    v.x + v.y + v.z
}

/// Cross product of two Vec3f inputs
pub fn cross(a: Vec3f, b: Vec3f) -> Vec3f {
    Vec3f::new(a.y * b.z - a.z * b.y,
               a.z * b.x - a.x * b.z,
               a.x * b.y - a.y * b.x)
}

/// Distance between two points
pub fn distance3(a: Vec3f, b: Vec3f) -> f32 {
    (a - b).magnitude()
}

/// Linearly interpolate between two points by the factor `t`. When
/// `t` is zero the result is `p0`, and when `t` is one the result is
/// `p1`. The range of `t` is not clamped.
pub fn lerp3(p0: &Vec3f, p1: &Vec3f, t: f32) -> Vec3f {
    let n = 1.0 - t;
    Vec3f::new(p0.x * n + p1.x * t,
               p0.y * n + p1.y * t,
               p0.z * n + p1.z * t)
}

impl Add for Vec3f {
    type Output = Vec3f;
    fn add(self, v: Vec3f) -> Vec3f {
        Vec3f::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Add<f32> for Vec3f {
    type Output = Vec3f;
    fn add(self, s: f32) -> Vec3f {
        Vec3f::new(self.x + s, self.y + s, self.z + s)
    }
}

impl Div for Vec3f {
    type Output = Vec3f;
    fn div(self, v: Vec3f) -> Vec3f {
        Vec3f::new(self.x / v.x, self.y / v.y, self.z / v.z)
    }
}

impl Mul<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn mul(self, v: Vec3f) -> Vec3f {
        Vec3f::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, f: f32) -> Vec3f {
        Vec3f::new(self.x * f, self.y * f, self.z * f)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;
    fn sub(self, v: Vec3f) -> Vec3f {
        Vec3f::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Sub<f32> for Vec3f {
    type Output = Vec3f;
    fn sub(self, s: f32) -> Vec3f {
        Vec3f::new(self.x - s, self.y - s, self.z - s)
    }
}

#[test]
fn test_vec3f_create() {
    vec3f(1.0f32, 2, 3i32);
    assert!(Vec3f::from_scalar(1) == vec3f(1, 1, 1));
}

#[test]
fn test_vec3f_magnitude() {
    let v = vec3f(-4, 0, 0);
    assert!(v.magnitude_squared() == 16.0);
    assert!(v.magnitude() == 4.0);
}

#[test]
fn test_vec3f_distance() {
    assert!(distance3(vec3f(0, -4, 0), vec3f(0, 4, 0)) == 8.0);
}

#[test]
fn test_vec3f_normalized() {
    assert_eq!(vec3f(0, 0, 0).normalized(), None);
    assert_eq!(vec3f(0.5, 0, 0).normalized(), Some(vec3f(1, 0, 0)));
}

#[test]
fn test_vec3f_project_onto() {
    let z = vec3f(0, 0, 0);
    let a = vec3f(1, 0, 0);
    let b = vec3f(1, 1, 0);
    assert_eq!(a.project_onto(z), None);
    assert_eq!(a.project_onto(b).unwrap(), vec3f(0.5, 0.5, 0));
}

#[test]
fn test_vec3f_arithmetic() {
    let a = vec3f(1, 2, 3);
    let b = vec3f(4, 5, 6);
    assert!(a + b == vec3f(5, 7, 9));
    assert!(a - b == vec3f(-3, -3, -3));
    assert!(a * b == vec3f(4, 10, 18));
    assert!(a / b == vec3f(0.25, 2.0 / 5.0, 0.5));
}

#[test]
fn test_vec3f_dot() {
    assert!(dot3(vec3f(1, 2, 3), vec3f(4, 5, 6)) == 32.0);
}

#[test]
fn test_vec3f_cross() {
    assert!(cross(vec3f(1, 0, 0), vec3f(0, 1, 0)) == vec3f(0, 0, 1));
}

#[test]
fn test_vec3f_lerp() {
    let p0 = vec3f(1.0, 2.0, 3.0);
    let p1 = vec3f(-1.0, -2.0, -3.0);
    assert!(lerp3(&p0, &p1, 0.0) == p0);
    assert!(lerp3(&p0, &p1, 0.25) == vec3f(0.5, 1.0, 1.5));
    assert!(lerp3(&p0, &p1, 0.5) == vec3f(0.0, 0.0, 0.0));
    assert!(lerp3(&p0, &p1, 1.0) == p1);
}
