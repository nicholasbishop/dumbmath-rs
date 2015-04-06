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

use std::cmp;

/// Inclusive range from min to max
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InclusiveRange<T: Copy + Ord> {
    pub min: T,
    pub max: T
}

impl<T: Copy + Ord> InclusiveRange<T> {
    /// Create an InclusiveRange range from min to max
    pub fn new(min: T, max: T) -> InclusiveRange<T> {
        assert!(min <= max);
        InclusiveRange { min: min, max: max }
    }
}

/// Create range covering the overlap between two ranges, or None if
/// there is no overlap.
pub fn range_clamp<T: Copy + Ord>(a: &InclusiveRange<T>,
                                  b: &InclusiveRange<T>)
                                  -> Option<InclusiveRange<T>> {
    let min = cmp::max(a.min, b.min);
    let max = cmp::min(a.max, b.max);

    if min < max {
        Some(InclusiveRange::new(min, max))
    }
    else {
        None
    }
}

#[test]
fn test_range_clamp() {
    assert!(range_clamp(&InclusiveRange::new(0, 2),
                        &InclusiveRange::new(0, 2)) ==
            Some(InclusiveRange::new(0, 2)));

    assert!(range_clamp(&InclusiveRange::new(0, 1),
                        &InclusiveRange::new(2, 3)) ==
            None);

    assert!(range_clamp(&InclusiveRange::new(2, 3),
                        &InclusiveRange::new(0, 1)) ==
            None);

    assert!(range_clamp(&InclusiveRange::new(0, 2),
                        &InclusiveRange::new(1, 3)) ==
            Some(InclusiveRange::new(1, 2)));

    assert!(range_clamp(&InclusiveRange::new(1, 3),
                        &InclusiveRange::new(0, 2)) ==
            Some(InclusiveRange::new(1, 2)));
}
