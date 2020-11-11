#![allow(unused)]
#![deny(unused_must_use)]

use common::prelude::*;
use async_std::net::SocketAddr;

pub struct Controller {

}

impl Actor for Controller {
    type Context = Context<Self>;
}

pub fn run(addr: SocketAddr) -> Addr<Controller> {

}