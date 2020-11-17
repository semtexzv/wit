#![allow(unused)]
#![deny(unused_must_use)]

use async_std::net::SocketAddr;
use std::net::ToSocketAddrs;

use common::prelude::*;
use common::{FunHash, FunctionUpdate, RuleUpdate, Function, Rule, Update, StaticData};
use common::prelude::sha2::Digest;
use common::prelude::quix::{DispatchError, process::{DynHandler, PidRecipient}};
use common::prelude::quix::memkv::{MemKv, Write, GlobalFind};
use common::prelude::quix::proto::Key;
use std::time::Duration;
use common::prelude::quix::util::uuid;

#[derive(DynHandler)]
#[dispatch(Update)]
pub struct Controller {
    router: Option<PidRecipient<Update>>,
}

impl Controller {
    pub fn start(addr: SocketAddr) -> Pid<Controller> {
        Process::start_with(|ctx| {
            MemKv::from_registry().do_send(Write {
                key: "/pids/controller".to_string().into_bytes(),
                value: ctx.pid().id().as_bytes().to_vec(),
            });

            ctx.spawn(wrap_future(handle(ctx.pid(), addr)));
            ctx.run_interval(Duration::from_secs(1), |this, ctx| {
                //  if this.router.is_none() {
                this.search_for_router(ctx);
                // }
            });
            Controller {
                router: None
            }
        })
    }
    fn search_for_router(&self, ctx: &mut Process<Controller>) {
        let f = wrap_future(MemKv::from_registry().send(GlobalFind { key: "/pids/router".to_string().into_bytes() }));
        let f = f.map(|r, this: &mut Self, ctx| r.unwrap().unwrap());
        let f = f.map(|r, this: &mut Self, ctx| {
            if let Some(r) = r {
                let pid = uuid(r);
                info!("Found the router with pid: {:?}", pid);
                this.router = Some(PidRecipient::from_id(pid));
            }
        });
        ctx.spawn(f);
    }
}

impl Actor for Controller {
    type Context = Process<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {}
}

impl Handler<Update> for Controller {
    type Result = Result<(), DispatchError>;

    fn handle(&mut self, msg: Update, ctx: &mut Self::Context) -> Self::Result {
        println!("Router update");
        self.router.as_ref().unwrap().do_send(msg).unwrap();
        Ok(())
    }
}

async fn post_function(mut req: tide::Request<Pid<Controller>>) -> tide::Result {
    let body = req.body_bytes().await?;
    let body = body;
    let id = sha2::Sha256::digest(&body);
    let id = FunHash(id.into());

    let msg = Update(StaticData {
        function: Some(Function {
            id: id.0.to_vec(),
            body,
        }),
        ..Default::default()
    });

    req.state().send(msg).await.unwrap().unwrap();

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

    let msg = Update(StaticData {
        rule: Some(Rule {
            spec: body.spec,
            funid: body.func.0.to_vec(),
        }),
        ..Default::default()
    });

    req.state().send(msg).await.unwrap().unwrap();

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