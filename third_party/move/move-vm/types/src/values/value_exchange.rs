// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::values::Value;

#[repr(transparent)]
pub struct ValueID(pub Value);

/// Trait which allows to swap values at (de)-serialization time.
pub trait ValueExchange {
    /// Returns a unique identifier as Move value. Additionally, records the mapping between
    /// the identifier and the given value for later reuse. For example, clients
    /// can serialize the value back and replace identifiers with values.
    /// Returns an error if a mapping already exists.
    ///
    /// Note: Identifier has exactly the same layout as recorded value.
    fn record_value(&self, value: Value) -> anyhow::Result<ValueID>;

    /// Returns the value based on the identifier. If such a mapping does not exist,
    /// an error is returned.
    fn claim_value(&self, id: ValueID) -> anyhow::Result<Value>;
}
