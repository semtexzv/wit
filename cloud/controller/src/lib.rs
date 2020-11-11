#![allow(unused)]
#![deny(unused_must_use)]

use common::prelude::*;
use async_std::net::SocketAddr;
use std::net::ToSocketAddrs;
use common::prelude::sha2::Digest;
use common::{Function, Rule, FunHash};
use common::prelude::dev::fut::wrap_future;

pub struct Controller {
    functions: Recipient<Function>,
    assignments: Recipient<Rule>,
}

impl Actor for Controller {
    type Context = Context<Self>;
}

impl Controller {
    pub fn start<R>(r: Addr<R>, addr: SocketAddr) -> Addr<Controller>
    where R: Actor<Context=Context<R>> + Handler<Function> + Handler<Rule>
    {
        Actor::create(|ctx| {
            ctx.spawn(wrap_future(handle(ctx.address(), addr)));
            Controller {
                functions: r.clone().recipient(),
                assignments: r.recipient(),
            }
        })
    }
}

impl Handler<Function> for Controller {
    type Result = ();

    fn handle(&mut self, msg: Function, ctx: &mut Self::Context) -> Self::Result {
        self.functions.do_send(msg).unwrap();
    }
}

impl Handler<Rule> for Controller {
    type Result = ();

    fn handle(&mut self, msg: Rule, ctx: &mut Self::Context) -> Self::Result {
        self.assignments.do_send(msg).unwrap()
    }
}

async fn post_function(mut req: tide::Request<Addr<Controller>>) -> tide::Result {
    let body = req.body_bytes().await?;
    let body = Bytes::from(body);
    let id = sha2::Sha256::digest(&body);
    let id = FunHash(id.into());

    req.state().send(Function {
        id: id.clone(),
        body,
    }).await.unwrap();

    Ok(tide::Response::builder(tide::http::StatusCode::Created)
        .body(tide::Body::from_json(&json::json!({ "id": id }))?)
        .content_type(tide::http::mime::BYTE_STREAM)
        .build()
    )
}

async fn post_assign(mut req: tide::Request<Addr<Controller>>) -> tide::Result {
    let mut body: Rule = req.body_json().await?;
    let x = req.state().send(body).await.unwrap();

    Ok(tide::Response::builder(tide::http::StatusCode::Created)
        .build()
    )
}

async fn handle(control: Addr<Controller>, addr: SocketAddr) {
    let mut app = tide::with_state(control);
    app.at("/functions").post(post_function);
    app.at("/rules").post(post_assign);
    app.listen(addr).await.unwrap();
}