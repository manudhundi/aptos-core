// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::values::{IntegerValue, Struct, Value, ValueExchange, ValueID};
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
    fn record_value(&self, value: Value) -> anyhow::Result<ValueID> {
        let mut data = self.data.borrow_mut();
        let key = data.len() as u64;
        match value.value_as::<IntegerValue>().unwrap() {
            IntegerValue::U64(x) => {
                data.insert(key, x as u128);
                Ok(ValueID(Value::u64(key)))
            },
            IntegerValue::U128(x) => {
                data.insert(key, x);
                Ok(ValueID(Value::u128(key as u128)))
            },
            _ => bail!("Unsupported value in tests"),
        }
    }

    fn claim_value(&self, id: ValueID) -> anyhow::Result<Value> {
        let data = self.data.borrow();
        match id.0.value_as::<IntegerValue>().unwrap() {
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

    let patched_blob = value.complicated_serialize(&layout, &exchange).unwrap();
    let patched_value = Value::simple_deserialize(&patched_blob, &layout).unwrap();
    let expected_patched_value = Value::struct_(Struct::pack(vec![
        Value::u64(100),
        Value::u128(0),
        Value::u64(1),
    ]));
    assert!(patched_value.equals(&expected_patched_value).unwrap());
    let patched_blob = patched_value.simple_serialize(&layout).unwrap();

    *exchange.data.borrow_mut().get_mut(&0).unwrap() += 100;
    *exchange.data.borrow_mut().get_mut(&1).unwrap() -= 100;

    let final_value = Value::complicated_deserialize(&patched_blob, &layout, &exchange).unwrap();

    let expected_final_value = Value::struct_(Struct::pack(vec![
        Value::u64(100),
        Value::u128(201),
        Value::u64(2),
    ]));
    assert!(expected_final_value.equals(&final_value).unwrap());
}
