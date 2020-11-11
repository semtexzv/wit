#![allow(unused)]
#![deny(unused_must_use)]

use common::prelude::*;
use worker::Worker;
use router::Router;

use gateway;

fn main() {
    actix::run(async move {
        let workers = SyncArbiter::start(8, || Worker {});
        let router = Router::start(Router::new(workers.recipient()));
        gateway::run(router.recipient()).await;
    }).unwrap();
}