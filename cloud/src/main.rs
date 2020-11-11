#![allow(unused)]
#![deny(unused_must_use)]

use common::prelude::*;
use worker::Worker;
use router::Router;

use gateway;
use controller::Controller;
use std::net::SocketAddr;

fn main() {
    actix::run(async move {
        let workers = SyncArbiter::start(8, || Worker {});
        let router = Router::start(Router::new(workers.recipient()));
        let control = Controller::start(router.clone(), SocketAddr::from(([127, 0, 0, 1], 9090)));

        gateway::run(router.recipient(), SocketAddr::from(([127, 0, 0, 1], 8080))).await;
    }).unwrap();
}