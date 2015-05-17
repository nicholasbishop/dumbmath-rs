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

use std::f32;
use vec3f::Vec3f;

pub struct Aabb3f {
    pub min: Vec3f,
    pub max: Vec3f
}

impl Aabb3f {
    /// Create an empty Aabb3f with min initialized to +infinity and
    /// max is initialized to -infinity.
    pub fn new() -> Aabb3f {
        Aabb3f {
            min: Vec3f::from_scalar(f32::INFINITY),
            max: Vec3f::from_scalar(f32::NEG_INFINITY)
        }
    }

    pub fn from_point(point: Vec3f) -> Aabb3f {
        Aabb3f {
            min: point,
            max: point
        }
    }

    /// True if the point intersects the box
    pub fn contains_point(&self, point: Vec3f) -> bool {
        (self.min.x <= point.x &&
         self.min.y <= point.y &&
         self.min.z <= point.z &&

         self.max.x >= point.x &&
         self.max.y >= point.y &&
         self.max.z >= point.z)
    }
}

#[test]
fn test_aabb3f_contains_point() {
    use vec3f::ZERO_3F;
    assert!(!Aabb3f::new().contains_point(ZERO_3F));
    assert!(Aabb3f::from_point(ZERO_3F).contains_point(ZERO_3F));
}
