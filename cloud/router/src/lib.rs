#![allow(unused)]
#![deny(unused_must_use)]

use common::prelude::*;
use std::collections::HashMap;
use common::{FunHash, Function, Event, Rule, Invoke, InvokeFun, FunctionUpdate, RuleUpdate, EventRecvd, InvokeResult};

#[derive(ProcessDispatch)]
#[dispatch(FunctionUpdate, RuleUpdate, EventRecvd)]
pub struct Router {
    functions: HashMap<FunHash, Vec<u8>>,
    assignments: HashMap<String, FunHash>,
    workers: Recipient<InvokeFun>,
}

impl Router {
    pub fn new(workers: Recipient<InvokeFun>) -> Self {
        Router {
            functions: Default::default(),
            assignments: Default::default(),
            workers,
        }
    }
}

impl Actor for Router {
    type Context = Process<Self>;
}

impl Handler<FunctionUpdate> for Router {
    type Result = ();

    fn handle(&mut self, msg: FunctionUpdate, ctx: &mut Process<Self>) -> Self::Result {
        self.functions.insert(FunHash::from(msg.0.id), msg.0.body);
    }
}

impl Handler<RuleUpdate> for Router {
    type Result = ();

    fn handle(&mut self, msg: RuleUpdate, ctx: &mut Process<Self>) -> Self::Result {
        self.assignments.insert(msg.0.spec, FunHash::from(msg.0.funid));
    }
}

impl Handler<EventRecvd> for Router {
    type Result = Response<InvokeResult, ()>;

    fn handle(&mut self, event: EventRecvd, ctx: &mut Process<Self>) -> Self::Result {
        info!("Router event");
        let fun = if let Some(fun) = self.assignments.get(&event.spec) {
            fun
        } else {
            return Response::reply(Err(()));
        };

        let body = if let Some(body) = self.functions.get(&fun) {
            body
        } else {
            return Response::reply(Err(()));
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
