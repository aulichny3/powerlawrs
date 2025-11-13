// Copyright (c) 2025 Adam Ulichny
//
// This source code is licensed under the MIT OR Apache-2.0 license
// that can be found in the LICENSE-MIT or LICENSE-APACHE files
// at the root of this source tree.

//! The generic Power-Law distribution defined by Cf(x) where f(x) = x^(-alpha) and C is the normalizing constant to ensure the distribution integrates to 1
//! where C = (alpha - 1) * x_min^(alpha - 1).
//! Continuous, Unbounded Power Law which simplifies to a Pareto Type I.
//! However, the alpha parameter will be exactly 1.0 greater than that of more
//! common expressions such as the Pareto Type I pdf listed at:
//! [https://en.wikipedia.org/wiki/Pareto_distribution](https://en.wikipedia.org/wiki/Pareto_distribution)

use super::Distribution;
use rand::prelude::*;

