#![allow(unused)]
#![deny(unused_must_use)]

pub mod prelude;

// pub mod proto {
//     //include!("./proto/wit.router.rs");
//     include!("./proto/wit.control.rs");
// }

use prelude::*;

#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FunHash([u8; 32]);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Function {
    pub id: FunHash,
    pub body: Bytes,
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct Assignment {
    pub spec: EventSpec,
    pub fun: FunHash,
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
    pub fun: Bytes,
    pub event: Event,
}