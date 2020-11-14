#![allow(unused)]
#![deny(unused_must_use)]

pub mod prelude;

pub mod proto {
    //include!("./proto/wit.router.rs");
    include!("./proto/wit.control.rs");
}

use prelude::*;
use serde::{Serializer, Deserializer};
use serde::de::Error;
use std::convert::TryInto;
use prost::DecodeError;
use bytes::{BufMut, Buf};

pub use proto::*;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FunHash(pub [u8;32]);

impl From<Vec<u8>> for FunHash {
    fn from(v: Vec<u8>) -> Self {
        Self(v.try_into().unwrap())
    }
}

impl Serialize for FunHash {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(&hex::encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for FunHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let res = String::deserialize(deserializer)
            .and_then(|string| hex::decode(&string).map_err(|err| D::Error::custom(err.to_string())))
            .and_then(|bytes| bytes.as_slice().try_into().map_err(|_| D::Error::custom("failed to deserialize public key")))?;

        Ok(FunHash(res))
    }
}
/*
#[derive(Message)]
#[rtype(result = "()")]
pub struct Function {
    pub id: FunHash,
    pub body: Bytes,
}

#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct Rule {
    pub spec: EventSpec,
    pub func: FunHash,
}

//#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub type EventSpec = String;

#[derive(Message)]
#[rtype(result = "Result<Bytes,()>")]
pub struct Event {
    pub spec: EventSpec,
    pub data: Bytes,
}

#[derive(Message)]
#[rtype(result = "Result<Bytes,()>")]
pub struct Invoke {
    pub funid: FunHash,
    pub fun: Bytes,
    pub event: Event,
}*/
