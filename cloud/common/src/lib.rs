#![allow(unused)]
#![deny(unused_must_use)]

pub mod prelude;

// pub mod proto {
//     //include!("./proto/wit.router.rs");
//     include!("./proto/wit.control.rs");
// }

use prelude::*;
use serde::{Serializer, Deserializer};
use serde::de::Error;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FunHash(pub [u8; 32]);

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
fn as_base64<S>(key: &FunHash, serializer: &mut S) -> Result<(), S::Error>
where S: Serializer
{
    serializer.serialize_str(&base64::encode(&key.0))
}

fn from_base64<'de, D>(deserializer: &mut D) -> Result<FunHash, D::Error>
where D: Deserializer<'de>
{
    use serde::de::Error;
    String::deserialize(deserializer)

}*/

#[derive(Message)]
#[rtype(result = "()")]
pub struct Function {
    pub id: FunHash,
    pub body: Bytes,
}

#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct Assignment {
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
}