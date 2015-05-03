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

use vector::Vec3f;

/// Line of infinite length represented by two distinct points it
/// passes through.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line3f {
    points: (Vec3f, Vec3f)
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
}
