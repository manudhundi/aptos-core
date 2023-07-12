// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::values::IntegerValue;
use move_binary_format::errors::PartialVMResult;

/// Represents a unique identifier. Implemented as a type to
/// avoid confusion with values it replaces.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ID(pub u64);

/// Trait which allows to generate unique per-block identifiers, as well as
/// record mappings between values and identifiers in a block.
pub trait GenID {
    /// Returns a unique identifier. Additionally, records the mapping between
    /// the identifier and the given value for later reuse. For example, clients
    /// can serialize the value back and replace identifiers with values.
    /// Returns an error if a mapping already exists.
    fn generate_id_and_record_value(&self, value: IntegerValue) -> PartialVMResult<ID>;

    /// Returns the value based on the identifier. Returns an error if such a
    /// mapping does not exist.
    fn get_value(&self, id: ID) -> PartialVMResult<IntegerValue>;
}
