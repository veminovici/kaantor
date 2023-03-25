# Simplee / Kaantor

BADGES_HERE

A project for distributed algorithms using the actor model.

## KAANTOR
A crate which implements the node actors for distributed algorithms. Each node can implement one or multiple
distributed algorithms. You can build a graph by connecting the actor nodes. Once the graph is built, you can run 
any of the implemented ditributed algorithms.  

### Build and Run the Examples
For a simple example (ping-pong) see [nexus.rs](./kaantor/examples/nexus.rs) in the *examples* folder under *kaantor* project. In the example, we create a tree where the node 1 is connected to nodes 2 and 3. We start a ping/pong protocol, by sending a PING message from node 1 to its neighbour nodes, 2 and 3. The two nodes, reponde back with a PONG.

```bsh
cargo build
RUST_LOG=debug cargo run --example nexus

INFO  Starting the example NEXUS_GET
INFO  A001 || RCVD | USER >> A001 | K010 | START 0012
INFO  A002 || RCVD | A001 >> A002 | K010 | PING 0012
INFO  A003 || RCVD | A001 >> A003 | K010 | PING 0012
INFO  A001 || RCVD | A002 >> A001 | K010 | PONG 0013
INFO  A001 || RCVD | A003 >> A001 | K010 | PONG 0013
```

## KAANTOR-GRAPH
A crate which implements a graph and tree data structures using adjacent lists. If you choose to use the *pretty* feature, you can pretty-print tree structures. 

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