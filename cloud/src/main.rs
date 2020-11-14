#![allow(unused)]
#![deny(unused_must_use)]

use common::prelude::*;
use worker::Worker;
use router::Router;

use gateway;
use controller::Controller;
use std::net::SocketAddr;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    actix::run(async move {
        let c = worker::modcache();
        let workers = SyncArbiter::start(2, move || Worker::new(c.clone()));
        let router = Process::start(Router::new(workers.recipient()));
        let control = Controller::start(router.clone(), SocketAddr::from(([127, 0, 0, 1], 9091)));

        gateway::run(router.recipient(), SocketAddr::from(([127, 0, 0, 1], 8080))).await;
    }).unwrap();
}