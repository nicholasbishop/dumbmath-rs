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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Degrees(f32);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Radians(f32);

impl Degrees {
    pub fn value(self) -> f32 {
        let Degrees(v) = self;
        v
    }

    pub fn to_radians(self) -> Radians {
        Radians(self.value() * (f32::consts::PI / 180.0))
    }
}

impl Radians {
    pub fn value(self) -> f32 {
        let Radians(v) = self;
        v
    }

    pub fn to_degrees(self) -> Degrees {
        Degrees(self.value() * (180.0 / f32::consts::PI))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32;

    #[test]
    fn test_conversions() {
        assert_eq!(Degrees(0.0).to_radians(), Radians(0.0));
        assert_eq!(Radians(0.0).to_degrees(), Degrees(0.0));

        assert_eq!(Degrees(360.0).to_radians(), Radians(2.0 * f32::consts::PI));
        assert_eq!(Radians(f32::consts::PI * 0.5).to_degrees(), Degrees(90.0));
    }
}
