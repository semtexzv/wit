use common::prelude::*;
use common::{Invoke, FunHash};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Worker {
    cache: HashMap<FunHash, executor::Module>,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            cache: Default::default()
        }
    }
}

impl Actor for Worker {
    type Context = SyncContext<Self>;
}

impl Handler<Invoke> for Worker {
    type Result = Result<Bytes, ()>;

    fn handle(&mut self, msg: Invoke, _ctx: &mut Self::Context) -> Self::Result {
        let fun: &mut executor::Module = match self.cache.entry(msg.funid) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(v) => {
                info!("Compiling module: {:?}", msg.funid);
                // TODO: Do compilation off the event path
                v.insert(executor::compile(&msg.fun).unwrap())
            }
        };
        info!("Event");
        let inst = fun.instantiate(&executor::imports()).unwrap();
        let res = executor::exec(inst, &[msg.event.data.as_ref()]).unwrap();
        return Ok(Bytes::from(res));
    }
}