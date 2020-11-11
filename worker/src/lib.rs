use common::prelude::*;
use common::Invoke;

pub struct Worker {}

impl Worker {}

impl Actor for Worker {
    type Context = SyncContext<Self>;
}

impl Handler<Invoke> for Worker {
    type Result = Result<Bytes, ()>;

    fn handle(&mut self, msg: Invoke, _ctx: &mut Self::Context) -> Self::Result {
        let res = executor::exec(msg.fun.as_ref(), &[msg.event.data.as_ref()]);
        return res.map_err(|_| ()).map(|v| Bytes::from(v))
    }
}