// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//! Historically, Move values were carrying all the data. Here, we introduce
//! "patched" Move values where a value can carry a unique identifier instead.
//! Unique identifiers can be used by clients to record data outside od the VM.
//! Interpretation of identifiers / values is done at (de-)serialization time.

pub mod identifiers;
pub mod serialization_type_layout;
#[cfg(test)]
mod values_tests;
