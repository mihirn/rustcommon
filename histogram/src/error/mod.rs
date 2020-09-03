// Copyright 2020 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum HistogramError {
    #[error("histogram contains no samples")]
    /// The histogram contains no samples.
    Empty,
    #[error("invalid percentile")]
    InvalidPercentile,
    #[error("value out of range")]
    /// The requested value is out of range.
    OutOfRange,
}
