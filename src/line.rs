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

use vector::{distance3, Vec3f};

/// Line between two points. Note that this type does not distinguish
/// between a line and a line segment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line3f {
    pub start: Vec3f,
    pub end: Vec3f
}

impl Line3f {
    pub fn new(start: &Vec3f, end: &Vec3f) -> Line3f {
        Line3f {
            start: *start,
            end: *end
        }
    }

    pub fn length(&self) -> f32 {
        distance3(self.start, self.end)
    }
}

#[test]
fn test_segment_length() {
    use vector::vec3f;
    let l = Line3f::new(&vec3f(0, 0, 0),
                        &vec3f(0, 0, 9));
    assert!(l.length() == 9.0);
}
