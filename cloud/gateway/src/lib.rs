#![allow(unused)]
#![deny(unused_must_use)]

use common::prelude::*;
use common::Event;
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::header::HOST;
use hyper::Body;


pub async fn run(rec: Recipient<Event>, addr: SocketAddr) {
    let rec = rec.clone();
    let svc = service_fn(move |req| {
        let rec = rec.clone();
        async move {
            let host = req.headers().get(HOST).expect("Missing host");
            let host = host.to_str().unwrap();
            let parts = host.rsplit(".");

            let spec = format!("/http/{}", parts.collect::<Vec<_>>().join("/"));
            let event = Event {
                spec,
                data: Bytes::from(hyper::body::to_bytes(req.into_body()).await.unwrap().to_vec()),
            };

            let res = rec.send(event).await
                .unwrap()
                .unwrap();

            //let req = hyper::Response::builder().

            let body = Body::from(res.to_vec());
            let resp = hyper::Response::new(body);
            Ok::<hyper::Response<_>, Infallible>(resp)
        }
    });

    let make_svc = make_service_fn(|_conn| {
        let svc = svc.clone();
        async move {
            Ok::<_, Infallible>(svc)
        }
    });

    let srv = hyper::Server::bind(&addr)
        .serve(make_svc);

    srv.await.unwrap();
}