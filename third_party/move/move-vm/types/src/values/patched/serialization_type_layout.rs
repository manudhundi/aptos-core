// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

/// Similar to `MoveTypeLayout`, but only used at runtime and by the loader to
/// serialize and deserialize Move values which can store identifiers.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SerializationTypeLayout {
    // Similar to `MoveTypeLayout`, support booleans, integers, etc.
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Address,
    Signer,
    Vector(Box<SerializationTypeLayout>),
    // For structs, it's enough to store their runtime representation unlike in
    // `MoveTypeLayout`.
    Struct(Vec<SerializationTypeLayout>),
    // Special markers which specify that this value is either an unsigned integer
    // or a unique identifier. Note that identifiers must have at most 64-bits!
    U64Marker,
    U128Marker,
}
