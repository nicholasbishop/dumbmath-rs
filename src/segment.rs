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

use vector::{distance3, dot3, Vec3f};

/// Line segment between two points
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment3f {
    pub start: Vec3f,
    pub end: Vec3f
}

impl Segment3f {
    pub fn new(start: &Vec3f, end: &Vec3f) -> Segment3f {
        Segment3f {
            start: *start,
            end: *end
        }
    }

    pub fn length(&self) -> f32 {
        distance3(self.start, self.end)
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

#[test]
fn test_segment_length() {
    use vector::vec3f;
    let l = Segment3f::new(&vec3f(0, 0, 0),
                           &vec3f(0, 0, 9));
    assert!(l.length() == 9.0);
}

#[test]
fn test_segment_closest_point_to_point() {
    
}
