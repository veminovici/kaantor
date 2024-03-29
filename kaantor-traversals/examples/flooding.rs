use actix::prelude::*;
use kaantor::{nexus, IntoActorId};
use kaantor_traversals::{FloodingNode, FloodingPld};
use log::info;

fn main() {
    env_logger::init();
    info!("Starting the FLOODING example");

    let _code = System::new().block_on(async {
        // STEP 1: Create the nodes
        let node1 = FloodingNode::build(1.into()).await;
        let node2 = FloodingNode::build(2.into()).await;
        let node3 = FloodingNode::build(3.into()).await;
        let node4 = FloodingNode::build(4.into()).await;
        let node5 = FloodingNode::build(5.into()).await;

        // STEP 2: Create the edges between the nodes
        let _ = nexus::add_edge::<FloodingPld>(node1.aid(), node2.aid()).await; // 1 - 2
        let _ = nexus::add_edge::<FloodingPld>(node1.aid(), node3.aid()).await; // 1 - 3
        let _ = nexus::add_edge::<FloodingPld>(node2.aid(), node4.aid()).await; // 2 - 4
        let _ = nexus::add_edge::<FloodingPld>(node4.aid(), node5.aid()).await; // 4 - 5
        let _ = nexus::add_edge::<FloodingPld>(node5.aid(), node3.aid()).await; // 3 - 5

        // STEP 3: Start the protocol
        let _ = node1.send(10.into(), FloodingPld::Start(12)).await;
    });

    info!("Done");
}
