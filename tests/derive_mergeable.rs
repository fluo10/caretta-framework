#![cfg(feature = "macros")]
use std::collections::{HashMap, HashSet};

use caretta_framework::util::Mergeable;

#[derive(Clone, Debug, PartialEq)]
struct MergeableTuple(Option<u8>);

impl Mergeable for MergeableTuple {
    fn merge(&mut self, other: Self) {
        if let Some(x) = other.0 {
            self.0.insert(x);
        }
    }
}
impl From<Option<u8>> for MergeableTuple {
    fn from(value: Option<u8>) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Mergeable)]
struct MergeableStruct {
    opt: MergeableTuple,
}

#[cfg(test)]
fn test() {
    let zero = MergeableStruct {
        opt: Some(0).into(),
    };
    let one = MergeableStruct {
        opt: Some(1).into(),
    };
    let none = MergeableStruct { opt: None.into() };
    let mut zero_with_one = zero.clone();
    zero_with_one.merge(one.clone());
    let mut none_with_zero = none.clone();
    none_with_zero.merge(zero.clone());
    let mut zero_with_none = zero.clone();
    zero_with_none.merge(none.clone());
    assert_eq!(zero_with_one.clone(), one.clone());
    assert_eq!(none_with_zero, zero.clone());
    assert_eq!(zero_with_none, zero.clone());
}
