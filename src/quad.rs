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

use vector::{Line2f, Vec2f};

/// 2D Quadrilateral
/// 
/// ```text
///          top
///     p3________p2
///      |        |
///      |        |
/// left |        | right
///      |        |
///      |________|
///     p0        p1
///        bottom
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quad2f {
    pub points: (Vec2f, Vec2f, Vec2f, Vec2f)
}

/// Return type for Quad2f::iblerp
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBLerpResult {
    NoSolution,
    /// One solutions (as parametric coordinate)
    OneSolution(Vec2f),
    /// Two solutions (as parametric coordinates)
    TwoSolutions(Vec2f, Vec2f),
    ManySolutions
}

/// Return type for Quad2f::inv_bilerp_u
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InvBilerpResult {
    NoSolution,
    /// One solutions (as parametric coordinate)
    OneSolution(f32),
    /// Two solutions (as parametric coordinates)
    TwoSolutions(f32, f32),
    ManySolutions
}

impl Quad2f {
    pub fn new(a: Vec2f, b: Vec2f, c: Vec2f, d: Vec2f) -> Quad2f {
        Quad2f {
            points: (a, b, c, d)
        }
    }

    // TODO(nicholasbishop): code dedup

    /// Inverse bilinear interpolation
    ///
    /// Adapted from stackoverflow.com/questions/808441
    pub fn iblerp(self, point: Vec2f) -> IBLerpResult {
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

    /// Calculate horizontal parametric coordinate from cartesian point.
    pub fn inv_bilerp_u(self, point: Vec2f) -> InvBilerpResult {
        let p0mp = self.points.0 - point;
        let p1mp = self.points.1 - point;
        let p0mp3 = self.points.0 - self.points.3;
        let p1mp2 = self.points.1 - self.points.2;

        let a = p0mp.cross(p0mp3);
        let b0 = p0mp.cross(p1mp2);
        let b1 = p1mp.cross(p0mp3);
        let b = (b0 + b1) / 2.0;
        let c = p1mp.cross(p1mp2);

        let den = a - (2.0 * b) + c;
        if den == 0.0 {
            let m = a - c;
            if m == 0.0 {
                if a == 0.0 {
                    InvBilerpResult::ManySolutions
                }
                else {
                    InvBilerpResult::NoSolution
                }
            }
            else {
                InvBilerpResult::OneSolution(a / m)
            }
        }
        else {
            let left = a - b;
            let right = (b.powi(2) - a*c).sqrt();
            let s0 = (left + right) / den;
            let s1 = (left - right) / den;
            InvBilerpResult::TwoSolutions(s0, s1)
        }
    }

    pub fn lerp_bottom(self, u: f32) -> Vec2f {
        self.points.0.lerp(self.points.1, u)
    }

    pub fn lerp_top(self, u: f32) -> Vec2f {
        self.points.3.lerp(self.points.2, u)
    }

    pub fn lerp_left(self, v: f32) -> Vec2f {
        self.points.0.lerp(self.points.3, v)
    }

    pub fn lerp_right(self, v: f32) -> Vec2f {
        self.points.1.lerp(self.points.2, v)
    }

    pub fn blerp(self, uv: Vec2f) -> Vec2f {
        let rb = self.points.0.lerp(self.points.1, uv.x);
        let rt = self.points.3.lerp(self.points.2, uv.x);
        rb.lerp(rt, uv.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use vector::vec2f;

    #[test]
    fn test_inv_bilerp_u() {
        let q = Quad2f::new(vec2f(0, 0),
                            vec2f(4, 0),
                            vec2f(4, 4),
                            vec2f(0, 4));
        assert_eq!(q.inv_bilerp_u(vec2f(2, 2)),
                   InvBilerpResult::OneSolution(0.5));
        assert_eq!(q.inv_bilerp_u(vec2f(1, 2)),
                   InvBilerpResult::OneSolution(0.25));
        assert_eq!(q.inv_bilerp_u(vec2f(1, 3)),
                   InvBilerpResult::OneSolution(0.25));
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
}
