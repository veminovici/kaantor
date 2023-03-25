# Simplee / Kaantor

[![Rust](https://github.com/veminovici/kaantor/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/veminovici/kaantor/actions/workflows/ci.yml)

A project for distributed algorithms using the actor model.

---

## KAANTOR
A crate which implements the node actors for distributed algorithms. Each node can implement one or multiple
distributed algorithms. You can build a graph by connecting the actor nodes. Once the graph is built, you can run 
any of the implemented ditributed algorithms.  

### Build and Run the Examples
For a simple example (ping-pong) see [ping_pong.rs](./kaantor/examples/ping_pong.rs) in the [examples](./kaantor/examples/) folder under *kaantor* project. In the example, we create a tree where the node 1 is connected to nodes 2 and 3. We start a ping/pong protocol, by sending a PING message from node 1 to its neighbour nodes, 2 and 3. The two nodes, reponde back with a PONG.

```bsh
cargo build
RUST_LOG=debug cargo run --example ping_pong

INFO  Starting the example NEXUS_GET
INFO  A001 || RCVD | USER >> A001 | K010 | START 0012
INFO  A002 || RCVD | A001 >> A002 | K010 | PING 0012
INFO  A003 || RCVD | A001 >> A003 | K010 | PING 0012
INFO  A001 || RCVD | A002 >> A001 | K010 | PONG 0013
INFO  A001 || RCVD | A003 >> A001 | K010 | PONG 0013
```

---

## KAANTOR-TRAVERSALS
A crate which provides implementations for several traversal algorithms.

### Flooding Algorithm
The implementation can be found at [flooding.rs](./kaantor-traversals/src/flooding.rs). The current implementation is a simplified one which does not take in account concurrent flooding sessions. You can see a full running example of the algorithm in [flooding.rs](./kaantor-traversals/examples/flooding.rs) under the [examples](./kaantor-traversals/examples/) folder.

```bsh
RUST_LOG=debug cargo run --example flooding

INFO Starting the FLOODING example
INFO A001 || RCVD | USER >> A001 | K010 | START 0012
INFO A002 || RCVD | A001 >> A002 | K010 | FORWARD 0012
INFO A003 || RCVD | A001 >> A003 | K010 | FORWARD 0012
INFO A001 || RCVD | A002 >> A001 | K010 | FORWARD 0012
INFO A001 || RCVD | A003 >> A001 | K010 | FORWARD 0012
```

---

## KAANTOR-DERIVE
A create which implements a set of derive macros.

### BuildNode derive macro
The **BuildNode** derive macro can be used to automatically generate a **build*8 function. The macro should be
added to any structure which represents a node. The macro must be followed by the **payload** attribute whihc should contain the list of supported payloads by the node. 

### IntoActorId derive macro
The **IntoActorId** derive macro can be used to automatically generate the implementation for the **IntoActorId*8 trait for a given node structure. The node structure must have a *aid* field of type *ActorId*.

## FromActorId derive macro
The **FromActorId** derive macro can be used to automatically generate the implementation for the *From<ActorID>* trait for a given node structure. The node structure must have a *aid* field of type *ActorId* and implement the *Default* trait as well.

### Build and Run the Example
For an example, see the [derive.rs](./kaantor-derive/examples/derive.rs).

```rust
#[derive(BuildNode, Default, IntoActorId, FromActorId)]
#[payload(MyPayloadA, MyPayloadB)]
struct MyActor {
   aid: ActorId,
};
```

### Build and Run the Example
```bsh
cargo build
RUST_LOG=debug cargo run --example derive
```

---

## KAANTOR-GRAPH
A crate which implements graph and tree data structures using adjacent lists. If you choose to use the *pretty* feature, you can pretty-print tree structures. 

### Build and Run the Examples
For an example, see [main.rs](./examples/tree/src/main.rs) in the *tree* examples folder. In the example we build a tree with 1 as root, which has to child nodes, 2 and 3. The node 2 has one child as well, node 3.

```bsh
cd examples/tree
cargo build
RUST_LOG=debug cargo run

SPANNING_TREE
└─ 1
   ├─ 2
   │  └─ 4
   └─ 3
```

## About

> Code designed and written on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.