#![allow(unused)]
#![deny(unused_must_use)]

use common::prelude::*;
use std::collections::HashMap;
use common::{FunHash, Function, Event, Rule, Invoke, InvokeFun, FunctionUpdate, RuleUpdate, EventRecvd, InvokeResult, Update};
use quix::DispatchError;
use common::prelude::quix::memkv::{MemKv, Write};

#[derive(DynHandler)]
#[dispatch(Update, EventRecvd)]
pub struct Router {
    functions: HashMap<FunHash, Vec<u8>>,
    rules: HashMap<String, FunHash>,
    workers: Recipient<InvokeFun>,
}

impl Router {
    pub fn new(workers: Recipient<InvokeFun>) -> Self {
        Router {
            functions: Default::default(),
            rules: Default::default(),
            workers,
        }
    }
}

impl Actor for Router {
    type Context = Process<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        MemKv::from_registry().do_send(Write {
            key: "/pids/router".to_string().into_bytes(),
            value: ctx.pid().id().as_bytes().to_vec(),
        });
        info!("Router starting");
    }
}

impl Handler<Update> for Router {
    type Result = Result<(), DispatchError>;

    fn handle(&mut self, mut msg: Update, ctx: &mut Process<Self>) -> Self::Result {
        if let Some(v) = msg.function.take() {
            self.functions.insert(FunHash::from(v.id), v.body);
        }
        if let Some(v) = msg.rule.take() {
            self.rules.insert(v.spec, FunHash::from(v.funid));
        }

        Ok(())
    }
}

impl Handler<EventRecvd> for Router {
    type Result = Response<InvokeResult, DispatchError>;

    fn handle(&mut self, event: EventRecvd, ctx: &mut Process<Self>) -> Self::Result {
        info!("Router event");
        let fun = if let Some(fun) = self.rules.get(&event.spec) {
            fun
        } else {
            return Response::reply(Ok(InvokeResult::default()));
        };

        let body = if let Some(body) = self.functions.get(&fun) {
            body
        } else {
            return Response::reply(Ok(InvokeResult::default()));
        };

        let work = self.workers.send(InvokeFun(Invoke {
            funid: fun.0.to_vec(),
            funbody: body.to_vec(),
            event: Some(event.0),
        }));

        Response::fut(work.map(|e| e.unwrap()))
    }
}

#[test]
fn test_router() {
    pub struct MockWorker {}
    impl Actor for MockWorker {
        type Context = Context<Self>;
    }

    impl Handler<Invoke> for MockWorker {
        type Result = Result<Bytes, ()>;

        fn handle(&mut self, msg: Invoke, ctx: &mut Self::Context) -> Self::Result {
            Ok(msg.event.data)
        }
    }

    actix::run(async move {
        let worker = MockWorker::start(MockWorker {});
        let router = Router::start(Router::new(worker.recipient()));

        router.send(Function { body: Bytes::new(), id: Default::default() }).await.unwrap();
        router.send(Rule { funid: Default::default(), spec: "/test".to_string() }).await.unwrap();

        let req = Bytes::from_static(&[0, 0]);
        let res = router.send(Event { spec: "/test".to_string(), data: req.clone() }).await.unwrap();
        assert_eq!(res, Ok(req.clone()));

        let res = router.send(Event { spec: "/miss".to_string(), data: req.clone() }).await.unwrap();
        assert_eq!(res, Err(()));
    }).unwrap();
}
