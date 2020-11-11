#![allow(unused)]
#![deny(unused_must_use)]

use common::prelude::*;
use std::collections::HashMap;
use common::{FunHash, Function, Event, Rule, EventSpec, Invoke};

pub struct Router {
    functions: HashMap<FunHash, Bytes>,
    assignments: HashMap<EventSpec, FunHash>,
    workers: Recipient<Invoke>,
}

impl Router {
    pub fn new(workers: Recipient<Invoke>) -> Self {
        Router {
            functions: Default::default(),
            assignments: Default::default(),
            workers,
        }
    }
}

impl Actor for Router {
    type Context = Context<Self>;
}

impl Handler<Function> for Router {
    type Result = ();

    fn handle(&mut self, msg: Function, ctx: &mut Context<Self>) -> Self::Result {
        self.functions.insert(msg.id, msg.body);
    }
}

impl Handler<Rule> for Router {
    type Result = ();

    fn handle(&mut self, msg: Rule, ctx: &mut Context<Self>) -> Self::Result {
        self.assignments.insert(msg.spec, msg.func);
    }
}

impl Handler<Event> for Router {
    type Result = Response<Bytes, ()>;

    fn handle(&mut self, event: Event, ctx: &mut Context<Self>) -> Self::Result {
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
        Response::fut(self.workers.send(Invoke {
            funid: fun.clone(),
            fun: body.clone(),
            event,
        }).map(|e| e.unwrap()))
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
        router.send(Rule { fun: Default::default(), spec: "/test".to_string() }).await.unwrap();

        let req = Bytes::from_static(&[0, 0]);
        let res = router.send(Event { spec: "/test".to_string(), data: req.clone() }).await.unwrap();
        assert_eq!(res, Ok(req.clone()));

        let res = router.send(Event { spec: "/miss".to_string(), data: req.clone() }).await.unwrap();
        assert_eq!(res, Err(()));
    }).unwrap();
}
