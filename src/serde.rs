//! `Serialize` and `Deserialize` implementations for `Void` behind the `serde` feature flag.

extern crate serde;
use serde::serde::{Serialize, Deserialize, de::Error};

use crate::{unreachable, Void};


impl Serialize for Void {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        unreachable(*self)
    }
}

impl<'d> Deserialize<'d> for Void {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'d> {
        Err(D::Error::custom("Cannot deserialize `Void`, it is uninhabited."))
    }
}
