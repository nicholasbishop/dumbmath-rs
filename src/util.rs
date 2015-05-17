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

/// Convert a numeric type to an f32
pub trait CastF32 { fn as_f32(self) -> f32; }

impl CastF32 for f32 { fn as_f32(self) -> f32 { self } }
impl CastF32 for f64 { fn as_f32(self) -> f32 { self as f32 } }

impl CastF32 for i8  { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for i16 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for i32 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for i64 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for isize { fn as_f32(self) -> f32 { self as f32 } }

impl CastF32 for u8  { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for u16 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for u32 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for u64 { fn as_f32(self) -> f32 { self as f32 } }
impl CastF32 for usize { fn as_f32(self) -> f32 { self as f32 } }
