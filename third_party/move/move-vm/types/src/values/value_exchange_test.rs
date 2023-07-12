// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::values::{IntegerValue, Struct, Value, ValueExchange};
use anyhow::bail;
use move_core_types::value::{MoveStructLayout::Runtime, MoveTypeLayout};
use std::{cell::RefCell, collections::BTreeMap};

#[cfg(test)]
#[derive(Debug, Default)]
struct TestExchange {
    data: RefCell<BTreeMap<u64, u128>>,
}

#[cfg(test)]
impl ValueExchange for TestExchange {
    fn record_value(&self, value: Value) -> anyhow::Result<Value> {
        let mut data = self.data.borrow_mut();
        let key = data.len() as u64;
        match value.value_as::<IntegerValue>().unwrap() {
            IntegerValue::U64(x) => {
                data.insert(key, x as u128);
                Ok(Value::u64(key))
            },
            IntegerValue::U128(x) => {
                data.insert(key, x);
                Ok(Value::u128(key as u128))
            },
            _ => bail!("Unsupported value in tests"),
        }
    }

    fn claim_value(&self, id: Value) -> anyhow::Result<Value> {
        let data = self.data.borrow();
        match id.value_as::<IntegerValue>().unwrap() {
            IntegerValue::U64(key) => Ok(Value::u64(*data.get(&key).unwrap() as u64)),
            IntegerValue::U128(key) => Ok(Value::u128(*data.get(&(key as u64)).unwrap())),
            _ => bail!("Unsupported value ID in tests"),
        }
    }
}

#[test]
fn test() {
    let exchange = TestExchange::default();

    let value = Value::struct_(Struct::pack(vec![
        Value::u64(100),
        Value::u128(101),
        Value::u64(102),
    ]));
    let layout = MoveTypeLayout::Struct(Runtime(vec![
        MoveTypeLayout::U64,
        MoveTypeLayout::Marked(Box::new(MoveTypeLayout::U128)),
        MoveTypeLayout::Marked(Box::new(MoveTypeLayout::U64)),
    ]));

    let blob = value.simple_serialize(&layout).unwrap();
    let patched_value = Value::complicated_deserialize(&blob, &layout, &exchange).unwrap();
    let expected_patched_value = Value::struct_(Struct::pack(vec![
        Value::u64(100),
        Value::u128(0),
        Value::u64(1),
    ]));
    assert!(patched_value.equals(&expected_patched_value).unwrap());

    let blob = patched_value
        .complicated_serialize(&layout, &exchange)
        .unwrap();
    let final_value = Value::simple_deserialize(&blob, &layout).unwrap();
    assert!(value.equals(&final_value).unwrap());
}
