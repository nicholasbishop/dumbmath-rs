// Copyright 2015 Nicholas Bishop
//
// Closest-points-between-lines method adapted from "Real-Time
// Collision Detection" by Christer Ericson, published by Morgan
// Kaufmann Publishers, Copyright 2005 Elsevier Inc
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

use vector::Vec3f;

/// Line of infinite length represented by two distinct points it
/// passes through.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line3f {
    pub points: (Vec3f, Vec3f)
}

impl Line3f {
    /// Create a new line. Return None if the input points are
    /// identical.
    pub fn new(a: Vec3f, b: Vec3f) -> Option<Line3f> {
        if a == b {
            None
        }
        else {
            Some(Line3f {
                points: (a, b)
            })
        }
    }

    /// Find the closest points between two lines. The result is a
    /// pair of parametric points, the first for `self` and the second
    /// for `line`. If the lines are parallel then None is
    /// returned.
    ///
    /// Adapted from "Real-Time Collision Detection" by Christer
    /// Ericson, published by Morgan Kaufmann Publishers, Copyright
    /// 2005 Elsevier Inc
    pub fn closest_points_between_lines(self,
                                        line: Line3f) -> Option<(f32, f32)> {
        let d1 = self.points.1 - self.points.0;
        let d2 = line.points.1 - line.points.0;
        let r = self.points.0 - line.points.0;

        let a = d1.dot(d1);
        let b = d1.dot(d2);
        let c = d1.dot(r);
        let e = d2.dot(d2);
        let f = d2.dot(r);

        let d = a * e - b * b;
        if d == 0.0 {
            None
        }
        else {
            let s = (b * f - c * e) / d;
            let t = (a * f - b * c) / d;
            Some((s, t))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use vector::vec3f;

    #[test]
    fn test_line3f() {
        let a = vec3f(0, 0, 0);
        let b = vec3f(1, 0, 0);
        assert_eq!(Line3f::new(a, a), None);
        assert_eq!(Line3f::new(a, b).unwrap(), Line3f { points: (a, b) } );
    }

    #[test]
    fn test_closest_points_between_lines() {
        let a = vec3f(0, 0, 0);
        let b = vec3f(4, 0, 0);

        let c = vec3f(2, 0, 0);
        let d = vec3f(2, 2, 0);

        let l1 = Line3f::new(a, b).unwrap();
        let l2 = Line3f::new(c, d).unwrap();

        // Intersection
        assert_eq!(l1.closest_points_between_lines(l2).unwrap(), (0.5, 0.0));

        // Coincident
        assert_eq!(l1.closest_points_between_lines(l1), None);

        // Parallel, not coincident
        let e = vec3f(0, 0, 1);
        let f = vec3f(4, 0, 1);
        let l3 = Line3f::new(e, f).unwrap();
        assert_eq!(l1.closest_points_between_lines(l3), None);

        // No intersection, not parallel
        let g = vec3f(0, 4, 1);
        let h = vec3f(0, 0, 1);
        let l4 = Line3f::new(g, h).unwrap();
        assert_eq!(l1.closest_points_between_lines(l4).unwrap(),
                   (0.0, 1.0));
    }
}
