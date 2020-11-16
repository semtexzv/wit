#![allow(unused)]
#![deny(unused_must_use)]

use common::prelude::*;
use async_std::net::SocketAddr;
use std::net::ToSocketAddrs;
use common::prelude::*;
use common::{FunHash, FunctionUpdate, RuleUpdate, Function, Rule};
use common::prelude::sha2::Digest;
use common::prelude::quix::{DispatchError, process::{DynHandler, PidRecipient}};

#[derive(DynHandler)]
#[dispatch(FunctionUpdate, RuleUpdate)]
pub struct Controller {
    functions: PidRecipient<FunctionUpdate>,
    assignments: PidRecipient<RuleUpdate>,
}

impl Actor for Controller {
    type Context = Process<Self>;
}

impl Controller {
    pub fn start<R>(r: Pid<R>, addr: SocketAddr) -> Pid<Controller>
    where R: Actor<Context=Process<R>> + DynHandler + Handler<FunctionUpdate> + Handler<RuleUpdate>
    {
        Process::start_with(|ctx| {
            ctx.spawn(wrap_future(handle(ctx.pid(), addr)));
            Controller {
                functions: r.clone().recipient(),
                assignments: r.recipient(),
            }
        })
    }
}

impl Handler<FunctionUpdate> for Controller {
    type Result = Result<(), DispatchError>;

    fn handle(&mut self, msg: FunctionUpdate, ctx: &mut Self::Context) -> Self::Result {
        self.functions.do_send(msg).unwrap();
        Ok(())
    }
}

impl Handler<RuleUpdate> for Controller {
    type Result = Result<(), DispatchError>;

    fn handle(&mut self, msg: RuleUpdate, ctx: &mut Self::Context) -> Self::Result {
        let sent = self.assignments.send(msg);
        ctx.spawn(wrap_future(async move {
            sent.await.unwrap().unwrap()
        }));
        Ok(())
    }
}

async fn post_function(mut req: tide::Request<Pid<Controller>>) -> tide::Result {
    let body = req.body_bytes().await?;
    let body = body;
    let id = sha2::Sha256::digest(&body);
    let id = FunHash(id.into());

    req.state().send(FunctionUpdate(Function {
        id: id.0.to_vec(),
        body,
    })).await.unwrap().unwrap();

    Ok(tide::Response::builder(tide::http::StatusCode::Created)
        .body(tide::Body::from_json(&json::json!({ "id": id }))?)
        .content_type(tide::http::mime::BYTE_STREAM)
        .build()
    )
}

#[derive(Debug, Deserialize)]
pub struct RuleApi {
    pub spec: String,
    pub func: FunHash,
}

async fn post_assign(mut req: tide::Request<Pid<Controller>>) -> tide::Result {
    let mut body: RuleApi = req.body_json().await?;
    let x = req.state().clone().send(RuleUpdate(Rule {
        spec: body.spec,
        funid: body.func.0.to_vec(),
    })).await.unwrap();

    Ok(tide::Response::builder(tide::http::StatusCode::Created)
        .build()
    )
}

async fn handle(control: Pid<Controller>, addr: SocketAddr) {
    let mut app = tide::with_state(control);
    app.at("/functions").post(post_function);
    app.at("/rules").post(post_assign);
    app.listen(addr).await.unwrap();
}