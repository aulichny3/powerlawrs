// Copyright (c) 2025 Adam Ulichny
//
// This source code is licensed under the MIT OR Apache-2.0 license
// that can be found in the LICENSE-MIT or LICENSE-APACHE files
// at the root of this source tree.

//! Represents the Pareto Type I density function parameterized by its alpha and minimum x value.
//! P(X=x) = x_min^alpha * x^(-1 - alpha) * alpha
//!
//! This is computationally equivalent to alpha * x_min.powf(alpha) / x.powf(alpha-1)cargo cle

pub mod estimation;
pub mod gof;
pub mod hypothesis;

pub use self::estimation::{find_alphas_exhaustive, find_alphas_fast};
pub use self::gof::gof;
pub use self::hypothesis::hypothesis_test;

use crate::dist::Distribution;