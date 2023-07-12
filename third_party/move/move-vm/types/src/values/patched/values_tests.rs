// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::values::{patched::identifiers::{GenID, ID}, IntegerValue, Value};
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::vm_status::StatusCode;
use std::{cell::RefCell, collections::BTreeMap};
use crate::values::patched::serialization_type_layout::SerializationTypeLayout;

#[cfg(test)]
#[derive(Debug)]
struct TestGenId {
    counter: RefCell<u64>,
    values: RefCell<BTreeMap<u64, u128>>,
}

#[cfg(test)]
impl TestGenId {
    fn new() -> Self {
        Self {
            counter: RefCell::new(0),
            values: RefCell::new(BTreeMap::new()),
        }
    }
}

#[cfg(test)]
impl GenID for TestGenId {
    fn generate_id_and_record_value(&self, value: IntegerValue) -> PartialVMResult<ID> {
        let id_value = *self.counter.borrow();
        if self
            .values
            .borrow_mut()
            .insert(id_value, value.cast_u128()?)
            .is_some()
        {
            return Err(PartialVMError::new(StatusCode::UNKNOWN_STATUS));
        }
        *self.counter.borrow_mut() += 1;
        Ok(ID(id_value))
    }

    fn get_value(&self, id: ID) -> PartialVMResult<IntegerValue> {
        self.values
            .borrow()
            .get(&id.0)
            .ok_or_else(|| PartialVMError::new(StatusCode::UNKNOWN_STATUS))
            .map(|x| IntegerValue::U128(*x))
    }
}

#[cfg(test)]
fn assert_equal_after_roundtrips(value: &Value, layout: &SerializationTypeLayout) {
    let generator = TestGenId::new();
    let blob =value.serialize_with_new_layout(layout).expect("Serialization failed during the test");
    let patched_value = Value::deserialize_with_new_layout_and_replace_markers(&blob, layout, &generator).expect("Deserialization failed during the test");

    let blob = patched_value.serialize_with_new_layout_and_replace_markers(layout, &generator).expect("Serialization failed during the test");
    let original_value = Value::deserialize_with_new_layout(&blob, layout).expect("Deserialization failed during the test");
    assert!(value.equals(&original_value).unwrap());
}

#[test]
fn test_no_replacement() {
    // Test integers.
    assert_equal_after_roundtrips(&Value::u8(1), &SerializationTypeLayout::U8);
    assert_equal_after_roundtrips(&Value::u16(2), &SerializationTypeLayout::U16);
    assert_equal_after_roundtrips(&Value::u32(3), &SerializationTypeLayout::U32);
    assert_equal_after_roundtrips(&Value::u64(4), &SerializationTypeLayout::U64);
    assert_equal_after_roundtrips(&Value::u128(5), &SerializationTypeLayout::U128);

    // Test marked values.
    assert_equal_after_roundtrips(&Value::u64(6), &SerializationTypeLayout::U64Marker);
    assert_equal_after_roundtrips(&Value::u128(7), &SerializationTypeLayout::U128Marker);

    // TODO: Add more tests.
}
