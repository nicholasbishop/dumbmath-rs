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

/// Vector with three f32 components
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3f {
    fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x: x,
                y: y,
                z: z }
    }
}

/// Vec3f(0.0, 0.0, 0.0)
pub const ZERO_3F: Vec3f = Vec3f { x: 0.0, y: 0.0, z: 0.0 };

pub trait CastF32 { fn as_f32(&self) -> f32; }

impl CastF32 for f32 { fn as_f32(&self) -> f32 { *self } }
impl CastF32 for f64 { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for i8  { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for i16 { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for i32 { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for u8  { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for u16 { fn as_f32(&self) -> f32 { *self as f32 } }
impl CastF32 for u32 { fn as_f32(&self) -> f32 { *self as f32 } }

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

impl Add for Vec3f {
    type Output = Vec3f;
    fn add(self, v: Vec3f) -> Vec3f {
        Vec3f::new(self.x + v.x, self.y + v.y, self.z + v.z)
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

#[test]
fn test_vec3f_create() {
    vec3f(1.0f32, 2, 3i32);
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
