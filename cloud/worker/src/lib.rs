use common::prelude::*;
use common::{FunHash, InvokeFun, InvokeResult};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::{Arc, Mutex};

pub type ModCache = Arc<Mutex<HashMap<FunHash, Arc<executor::Module>>>>;

pub fn modcache() -> ModCache {
    Default::default()
}

pub struct Worker {
    cache: ModCache
}

impl Worker {
    pub fn new(c: ModCache) -> Self {
        Self {
            cache: c
        }
    }
}

impl Actor for Worker {
    type Context = SyncContext<Self>;
}

impl Handler<InvokeFun> for Worker {
    type Result = Result<InvokeResult, ()>;

    fn handle(&mut self, msg: InvokeFun, _ctx: &mut Self::Context) -> Self::Result {
        let module: Arc<executor::Module> = {
            match self.cache.lock().unwrap().entry(FunHash::from(msg.0.funid)) {
                Entry::Occupied(e) => e.get().clone(),
                Entry::Vacant(v) => {
                    info!("Compiling module: {:?}", v.key());
                    let module = executor::compile(&msg.0.funbody).unwrap();
                    // TODO: Do compilation off the event path
                    v.insert(Arc::new(module)).clone()
                }
            }
        };

        info!("Event");
        let inst = module.instantiate(&executor::imports()).unwrap();
        let res = executor::exec(inst, &[msg.0.event.unwrap().data.as_ref()]).unwrap();
        return Ok(InvokeResult { response: res });
    }
}