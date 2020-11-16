#![allow(unused)]
#![deny(unused_must_use)]

use common::prelude::*;
use worker::Worker;
use router::Router;

use gateway;
use controller::Controller;
use std::net::SocketAddr;
use std::sync::mpsc::channel;
use common::prelude::quix::node::{NodeController, NodeConfig, Connect};
use common::prelude::quix::global::{Global, Set};
use std::str::FromStr;

struct Config {
    listen: SocketAddr,
    connect: Option<SocketAddr>,
    sc: String,
}

fn config() -> Config {
    let mut args = std::env::args();
    let _ = args.next();

    let sc = args.next().expect("Expected subcommand");

    let mut res = Config {
        listen: SocketAddr::from(([127, 0, 0, 1], 9090)),
        connect: None,
        sc,
    };

    match args.next().unwrap().as_str() {
        "-l" | "--listen" => {
            res.listen = SocketAddr::from_str(&args.next().unwrap()).unwrap();
        }
        "-c" | "--connect" => {
            res.connect = Some(SocketAddr::from_str(&args.next().unwrap()).unwrap())
        }
        "-h" | "--help" => {
            std::process::exit(0)
        }
        _ => {
            std::process::exit(1)
        }
    }
    res
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    actix::run(async move {
        let conf = config();

        Global::<NodeConfig>::from_registry().send(Set(NodeConfig {
            id: Uuid::new_v4(),
            listen: conf.listen,
        })).await.unwrap();

        if let Some(c) = conf.connect {
            NodeController::from_registry().send(Connect { addr: c }).await.unwrap();
        }

        match conf.sc.as_str() {
            "router" => {
                let c = worker::modcache();
                let workers = SyncArbiter::start(2, move || Worker::new(c.clone()));
                let router = Process::start(Router::new(workers.recipient()));
            }
            "control" => {
                Controller::start(SocketAddr::from(([127, 0, 0, 1], 1010)));
            }
        }


        tx.send(router.clone()).unwrap();
        gateway::run(router.recipient(), SocketAddr::from(([127, 0, 0, 1], 8080))).await;
    }).unwrap();
});