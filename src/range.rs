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

use std::ops::Sub;

/// Convenience type: InclusiveRange<f32>
pub type Rangef = InclusiveRange<f32>;

/// Inclusive range from min to max
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InclusiveRange<T: Copy + PartialOrd + Sub> {
    pub min: T,
    pub max: T
}

pub const RANGE_0_1_F32: InclusiveRange<f32> = InclusiveRange { min: 0.0, max: 1.0 };

fn pomax<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn pomin<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

impl<T: Copy + PartialOrd + Sub<Output=T>> InclusiveRange<T> {
    /// Create an InclusiveRange from min to max. Asserts that min is
    /// less than max.
    pub fn new(min: T, max: T) -> InclusiveRange<T> {
        assert!(min <= max);
        InclusiveRange { min: min, max: max }
    }

    /// Create an InclusiveRange by sorting the inputs.
    pub fn from_sorting(a: T, b: T) -> InclusiveRange<T> {
        if a <= b {
            InclusiveRange { min: a, max: b }
        }
        else {
            InclusiveRange { min: b, max: a }
        }
    }

    /// True if `min == max`, false otherwise.
    pub fn empty(self) -> bool {
        self.min == self.max
    }

    /// Distance between self.min and self.max
    pub fn length(self) -> T {
        self.max - self.min
    }

    /// Expand `self` as needed to include another range.
    pub fn expand(&mut self, other: InclusiveRange<T>) {
        self.min = pomin(self.min, other.min);
        self.max = pomax(self.max, other.max);
    }
}

/// Create range covering the overlap between two ranges, or None if
/// there is no overlap.
pub fn range_clamp<T: Copy + PartialOrd + Sub<Output=T>>
    (a: InclusiveRange<T>, b: InclusiveRange<T>)
     -> Option<InclusiveRange<T>> {
    let min = pomax(a.min, b.min);
    let max = pomin(a.max, b.max);

    if min <= max {
        Some(InclusiveRange::new(min, max))
    }
    else {
        None
    }
}

/// Create range covering both ranges (and any gap between them).
pub fn range_combine<T: Copy + PartialOrd + Sub<Output=T>>(a: InclusiveRange<T>,
                                                           b: InclusiveRange<T>)
                                                           -> InclusiveRange<T> {
    InclusiveRange::new(pomin(a.min, b.min),
                        pomax(a.max, b.max))
}

#[test]
fn test_from_sorting() {
    assert_eq!(InclusiveRange::from_sorting(0, 1),
               InclusiveRange::new(0, 1));
    assert_eq!(InclusiveRange::from_sorting(1, 1),
               InclusiveRange::new(1, 1));
    assert_eq!(InclusiveRange::from_sorting(1, 0),
               InclusiveRange::new(0, 1));
}

#[test]
fn test_empty() {
    assert_eq!(InclusiveRange::new(0, 1).empty(), false);
    assert_eq!(InclusiveRange::new(0, 0).empty(), true);
    assert_eq!(InclusiveRange::new(0.5, 0.5).empty(), true);
}

#[test]
fn test_range_clamp() {
    assert!(range_clamp(InclusiveRange::new(0, 2),
                        InclusiveRange::new(0, 2)) ==
            Some(InclusiveRange::new(0, 2)));

    assert!(range_clamp(InclusiveRange::new(0, 1),
                        InclusiveRange::new(2, 3)) ==
            None);

    assert!(range_clamp(InclusiveRange::new(2, 3),
                        InclusiveRange::new(0, 1)) ==
            None);

    assert!(range_clamp(InclusiveRange::new(0, 2),
                        InclusiveRange::new(1, 3)) ==
            Some(InclusiveRange::new(1, 2)));

    assert!(range_clamp(InclusiveRange::new(1, 3),
                        InclusiveRange::new(0, 2)) ==
            Some(InclusiveRange::new(1, 2)));

    assert!(range_clamp(InclusiveRange::new(1.0, 3.0),
                        InclusiveRange::new(0.0, 2.0)) ==
            Some(InclusiveRange::new(1.0, 2.0)));
}

#[test]
fn test_range_expand() {
    let mut r = InclusiveRange::new(0, 1);
    r.expand(InclusiveRange::new(-1, 2));
    assert!(r == InclusiveRange::new(-1, 2));
}

#[test]
fn test_range_combine() {
    assert!(range_combine(InclusiveRange::new(0, 2),
                          InclusiveRange::new(-2, 1)) ==
            InclusiveRange::new(-2, 2));
}

#[test]
fn test_range_length() {
    assert!(InclusiveRange::new(-1, 2).length() == 3);
    assert!(InclusiveRange::new(-1.0, 2.5).length() == 3.5);
}
