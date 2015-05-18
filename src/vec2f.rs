// Copyright 2015 Nicholas Bishop
//
// Orientation code adapted from "Real-Time Collision Detection" by
// Christer Ericson, published by Morgan Kaufmann Publishers,
// Copyright 2005 Elsevier Inc
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
use util::CastF32;
use vec3f::Vec3f;

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

    pub fn distance_squared(self, other: Vec2f) -> f32 {
        (self - other).magnitude_squared()
    }

    pub fn distance(self, other: Vec2f) -> f32 {
        self.distance_squared(other).sqrt()
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

pub fn detf_2x2(top_left: f32, top_right: f32,
                bot_left: f32, bot_right: f32) -> f32 {
    top_left * bot_right - top_right * bot_left
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

    pub fn cart_from_para(self, t: f32) -> Vec2f {
        self.points.0.lerp(self.points.1, t)
    }

    /// If result is greater than zero `point` lies to the left of the
    /// line. If result is less than zero `point` lies to the
    /// right. If result is zero, the point is on the line.
    ///
    /// The value is also twice the signed area of the triangle
    /// `(points.0, points.1, point)` (positive if counterclockwise,
    /// otherwise negative).
    ///
    /// Adapted from "Real-Time Collision Detection" by Christer
    /// Ericson, published by Morgan Kaufmann Publishers, Copyright
    /// 2005 Elsevier Inc
    pub fn orient(self, point: Vec2f) -> f32 {
        detf_2x2(self.points.0.x - point.x, self.points.0.y - point.y,
                 self.points.1.x - point.x, self.points.1.y - point.y)
    }
}

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
