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

use vector;

/// Sphere represented by a center and radius
#[derive(Clone, Copy, PartialEq)]
pub struct Sphere3f {
    pub center: vector::Vec3f,
    pub radius: f32
}

/// Sphere centered at (0.0, 0.0, 0.0) with a radius of 1.0
pub const UNIT: Sphere3f = Sphere3f { center: vector::ZERO_3F, radius: 1.0 };

impl Sphere3f {
    /// Create a Sphere3f from center point and radius
    pub fn new(center: vector::Vec3f, radius: f32) -> Sphere3f {
        Sphere3f {
            center: center,
            radius: radius
        }
    }

    /// Create a Sphere3f centered at zero with given radius
    pub fn from_radius(radius: f32) -> Sphere3f {
        Sphere3f::new(vector::ZERO_3F, radius)
    }

    /// Squared radius of the sphere
    pub fn radius_squared(&self) -> f32 {
        self.radius * self.radius
    }
}

#[test]
fn test_sphere3f_create() {
    Sphere3f::new(vector::vec3f(1, 2, 3), 4.0);
    Sphere3f::from_radius(1.0);
}

#[test]
fn test_sphere3f_radius_squared() {
    let r = 2.2f32;
    assert!(Sphere3f::from_radius(r).radius_squared() == (r * r));
}
