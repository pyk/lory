// Copyright (c) 2020, Bayu Aldi Yansyah <bayualdiyansyah@gmail.com>
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

/// A prelude for crates using the `lory` crate.
///
/// This prelude is similar to the standard library's prelude in that you'll
/// almost always want to import its entire contents, but unlike the standard
/// library's prelude you'll have to do so manually.
///
/// An example of using this is:
///
/// ```
/// use lory::prelude::*;
/// ```
///
/// We may add items to this over time as they become ubiquitous as well, but
/// otherwise this should help cut down on futures-related imports when you're
/// working with the `lory` crate!
///
pub use crate::user::*;
pub use crate::*;
