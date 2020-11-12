# Mission statement
Make cloud computing truly independent from operating system, hardware architecture, and location.

# How ?
Deliver performant edge platform. Extend with  additional functionality until the edge disappears.


## Design
- Actor model
- Individual component modelled as independent actors
- No central message broker
- Functions are stateless, implemented as WASM modules

## Platform requirements
The underlying distribution platform will be [quix](https://github.com/semtexzv/quix). But we need several features:
 
- Process migration
- Encryption
- Global KV Stores
- Consensus (Raft) 