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

    pub fn cross(self, other: Vec2f) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn lerp(self, other: Vec2f, t: f32) -> Vec2f {
        self * (1.0 - t) + other * t
    }

    pub fn dot(self, other: Vec2f) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn vec3f(self) -> Vec3f {
        Vec3f::new(self.x, self.y, 0.0)
    }

    pub fn magnitude_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(self) -> Option<Vec2f> {
        let m = self.magnitude();
        if m == 0.0 {
            None
        }
        else {
            Some(Vec2f::new(self.x / m, self.y / m))
        }
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

    pub fn closest_parametric_point(self, point: Vec2f) -> f32 {
        let p0p = point - self.points.0;
        let p0p1 = self.points.1 - self.points.0;

        p0p.dot(p0p1) / p0p1.dot(p0p1)
    }
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

    /// Inner product
    pub fn dot(self, v: Vec3f) -> f32 {
        let m = self * v;
        m.x + m.y + m.z
    }

    /// Cross product
    pub fn cross(self, v: Vec3f) -> Vec3f {
        Vec3f::new(self.y * v.z - self.z * v.y,
                   self.z * v.x - self.x * v.z,
                   self.x * v.y - self.y * v.x)
    }

    /// Distance to another point
    pub fn distance(self, v: Vec3f) -> f32 {
        (self - v).magnitude()
    }

    /// Linearly interpolate between two points by the factor
    /// `t`. When `t` is zero the result is `self`, and when `t` is
    /// one the result is `p`. The range of `t` is not clamped.
    pub fn lerp(self, p: Vec3f, t: f32) -> Vec3f {
        let n = 1.0 - t;
        Vec3f::new(self.x * n + p.x * t,
                   self.y * n + p.y * t,
                   self.z * n + p.z * t)
    }

    /// Squared length of the vector
    pub fn magnitude_squared(self) -> f32 {
        self.dot(self)
    }

    /// Length of the vector
    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    /// Create a normalized copy, or None if the magnitude is zero
    pub fn normalized(self) -> Option<Vec3f> {
        let m = self.magnitude();
        if m == 0.0 {
            None
        }
        else {
            let f = 1.0 / m;
            Some(self * f)
        }
    }

    /// Projection of `self` into `v`, or None if the magnitude of
    /// `b` is zero.
    pub fn project_onto(self, v: Vec3f) -> Option<Vec3f> {
        if v == ZERO_3F {
            None
        }
        else {
            Some(v * (self.dot(v) / v.magnitude_squared()))
        }
    }
}

/// Vec3f(0.0, 0.0, 0.0)
pub const ZERO_3F: Vec3f = Vec3f { x: 0.0, y: 0.0, z: 0.0 };

/// Convert a numeric type to an f32
pub trait CastF32 { fn as_f32(self) -> f32; }

impl CastF32 for f32 { fn as_f32(self) -> f32 { self } }
impl CastF32 for f64 { fn as_f32(self) -> f32 { self as f32 } }

impl CastF32 for i8  { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for i16 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for i32 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for i64 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for isize { fn as_f32(self) -> f32 { self as f32 } }

impl CastF32 for u8  { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for u16 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for u32 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for u64 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for usize { fn as_f32(self) -> f32 { self as f32 } }

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

#[cfg(test)]
mod test {
    use super::*;

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
        assert_eq!(vec3f(0, -4, 0).distance(vec3f(0, 4, 0)), 8.0);
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
        assert_eq!(vec3f(1, 2, 3).dot(vec3f(4, 5, 6)), 32.0);
    }

    #[test]
    fn test_vec3f_cross() {
        assert_eq!(vec3f(1, 0, 0).cross(vec3f(0, 1, 0)), vec3f(0, 0, 1));
    }

    #[test]
    fn test_vec3f_lerp() {
        let p0 = vec3f(1.0, 2.0, 3.0);
        let p1 = vec3f(-1.0, -2.0, -3.0);
        assert_eq!(p0.lerp(p1, 0.0), p0);
        assert_eq!(p0.lerp(p1, 0.25), vec3f(0.5, 1.0, 1.5));
        assert_eq!(p0.lerp(p1, 0.5), vec3f(0.0, 0.0, 0.0));
        assert_eq!(p0.lerp(p1, 1.0), p1);
    }
}
