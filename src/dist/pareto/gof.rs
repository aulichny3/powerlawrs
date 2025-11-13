// Copyright (c) 2025 Adam Ulichny
//
// This source code is licensed under the MIT OR Apache-2.0 license
// that can be found in the LICENSE-MIT or LICENSE-APACHE files
// at the root of this source tree.

//! Repeatedly KS test a series of x_min and alpha parameter sets and
//! the sample data to find the pair that has the best KS statistic. This is the
//! method proposed in Section 3.3 of
//! Clauset, Aaron and Shalizi, Cosma Rohilla and Newman, M. E. J. [doi:10.48550/ARXIV.0706.1062](https://doi.org/10.48550/arXiv.0706.1062)