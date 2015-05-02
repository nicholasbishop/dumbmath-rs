// Copyright 2015 Nicholas Bishop
//
// Closest-point method adapted from "Real-Time Collision Detection"
// by Christer Ericson, published by Morgan Kaufmann Publishers,
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

use vector::{distance3, dot3, lerp3, Vec3f};
use range::InclusiveRange;

pub type Rangef = InclusiveRange<f32>;

/// Line segment between two points
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment3f {
    pub start: Vec3f,
    pub end: Vec3f
}

impl Segment3f {
    /// Create a segment between two points
    pub fn new(start: &Vec3f, end: &Vec3f) -> Segment3f {
        Segment3f {
            start: *start,
            end: *end
        }
    }

    /// Get vector from start to end (not normalized)
    pub fn to_vec3f(&self) -> Vec3f {
        self.end - self.start
    }

    /// Length of the line segment
    pub fn length(&self) -> f32 {
        distance3(self.start, self.end)
    }

    /// Convert a distance in coordinate space to a distance in the
    /// line segment's parametric space. The sign of the input is
    /// kept.
    pub fn distance_to_parametric_delta(&self, distance: f32) -> f32 {
        distance / self.length()
    }

    /// Convert a parametric delta to coordinate space. The sign of
    /// the input is kept.
    pub fn distance_from_parametric_delta(&self, delta: f32) -> f32 {
        delta * self.length()
    }

    /// Linearly interpolate between the segment's endpoints by the
    /// factor `t`. When `t` is zero the result is `self.start`, and
    /// when `t` is one the result is `self.end`. The range of `t` is
    /// not clamped.
    pub fn point_from_parametric(&self, t: f32) -> Vec3f {
        lerp3(&self.start, &self.end, t)
    }

    /// Treat the range's start and end as parametric coords. Use
    /// `point_from_parametric` to interpolate the range into a new
    /// segment. The output segment is not clamped.
    pub fn segment_from_parametric_range(&self, r: Rangef) -> Segment3f {
        Segment3f::new(&self.point_from_parametric(r.min),
                       &self.point_from_parametric(r.max))
    }

    /// Project another segment onto `self`. The result is a
    /// parametric range of `self` clamped to [0, 1].
    pub fn project_segment_as_range(&self, other: &Segment3f) -> Rangef {
        Rangef::from_sorting(self.closest_point_to_point(&other.start).0,
                             self.closest_point_to_point(&other.end).0)
    }

    /// Find the point on the segment closest to the input point. The
    /// return value contains both the parametric and actual location
    /// of the closest point.
    ///
    /// Adapted from "Real-Time Collision Detection" by Christer
    /// Ericson, published by Morgan Kaufmann Publishers, Copyright
    /// 2005 Elsevier Inc
    pub fn closest_point_to_point(&self, point: &Vec3f) -> (f32, Vec3f) {
        let a = self.start;
        let b = self.end;

        let ab = b - a;
        // Project point onto ab, but deferring divide by dot3(ab, ab)
        let t = dot3(*point - a, ab);
        if t <= 0.0f32 {
            // point projects outside the [a,b] interval, on the a
            // side; clamp to a
            (0.0f32, a)
        } else {
            // Always nonnegative since denom = ||ab|| ∧ 2
            let denom = dot3(ab, ab); 
            if t >= denom {
                // point projects outside the [a,b] interval, on the b
                // side; clamp to b
                (1.0f32, b)
            } else {
                // point projects inside the [a,b] interval; must do
                // deferred divide now
                (t / denom, a + ab * t)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use vector::vec3f;

    fn make_seg(a: i32, b: i32) -> Segment3f {
        Segment3f::new(&vec3f(a, 0, 0), &vec3f(b, 0, 0))
    }

    #[test]
    fn test_to_vec3f() {
        assert_eq!(Segment3f::new(&vec3f(0, 0, 0),
                                  &vec3f(2, 3, 4)).to_vec3f(),
                   vec3f(2, 3, 4));
        
    }

    #[test]
    fn test_segment_length() {
        let s = Segment3f::new(&vec3f(0, 0, 0),
                               &vec3f(0, 0, 9));
        assert_eq!(s.length(), 9.0);
    }

    #[test]
    fn test_segment_distance_conversion() {
        let s = Segment3f::new(&vec3f(0, 1, 0),
                               &vec3f(0, 7, 0));
        
        let inputs = [
            (0.0f32, 0.0f32),
            (6.0, 1.0),
            (12.0, 2.0),
            (-3.0, -0.5)];

        for &(distance, delta) in inputs.iter() {
            assert_eq!(s.distance_to_parametric_delta(distance), delta);
            assert_eq!(s.distance_from_parametric_delta(delta), distance);
        }
    }

    #[test]
    fn test_point_from_parametric() {
        let s = Segment3f::new(&vec3f(0, 0, -1),
                               &vec3f(0, 0, 3));
        assert_eq!(s.point_from_parametric(0.0), vec3f(0, 0, -1));
        assert_eq!(s.point_from_parametric(1.0), vec3f(0, 0, 3));
        assert_eq!(s.point_from_parametric(0.5), vec3f(0, 0, 1));
    }

    #[test]
    fn test_segment_from_parametric_range() {
        let s = make_seg(0, 4);
        assert_eq!(s.segment_from_parametric_range(Rangef::new(0.0, 1.0)),
                   make_seg(0, 4));
        assert_eq!(s.segment_from_parametric_range(Rangef::new(0.25, 0.75)),
                   make_seg(1, 3));
    }

    #[test]
    fn test_segment_closest_point_to_point() {
        let s = Segment3f::new(&vec3f(2, 0, 0),
                               &vec3f(3, 0, 0));
        assert_eq!(s.closest_point_to_point(&vec3f(1, 0, 0)),
                   (0.0, vec3f(2, 0, 0)));
        assert_eq!(s.closest_point_to_point(&vec3f(4, 0, 0)),
                   (1.0, vec3f(3, 0, 0)));
        assert_eq!(s.closest_point_to_point(&vec3f(2.5, 1, 0)),
                   (0.5, vec3f(2.5, 0, 0)));
    }

    #[test]
    fn test_project_segment_as_range() {
        let s = make_seg(0, 4);
        assert_eq!(s.project_segment_as_range(&make_seg(0, 4)),
                   Rangef::new(0.0, 1.0));
        assert_eq!(s.project_segment_as_range(&make_seg(-1, 5)),
                   Rangef::new(0.0, 1.0));
        assert_eq!(s.project_segment_as_range(&make_seg(-1, -1)),
                   Rangef::new(0.0, 0.0));
        assert_eq!(s.project_segment_as_range(&make_seg(1, 3)),
                   Rangef::new(0.25, 0.75));
    }
}
