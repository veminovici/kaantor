use actix::prelude::*;
use kaantor::{nexus, IntoActorId};
use kaantor_traversals::{FloodingNode, FloodingPld};
use log::info;

fn main() {
    env_logger::init();
    info!("Starting the example NEXUS_GET");

    // async fn create(aid: ActorId) -> Node<MyActor> {
    //     let node = MyActor::from(aid);
    //     let addr = node.start();
    //     let node = Node::new(aid, addr);
    //     let _ = node.register_proxy::<MyPayload>().await;
    //     let _ = node.register_proxy::<MyPayload2>().await;
    //     node
    // }

    // initialize system
    let _code = System::new().block_on(async {
        // STEP 1: Create the nodes
        let node1 = FloodingNode::build(1.into()).await;
        let node2 = FloodingNode::build(2.into()).await;
        let node3 = FloodingNode::build(3.into()).await;

        // STEP 2: Create the edges between the nodes
        let _ = nexus::add_edge::<FloodingPld>(node1.aid(), node2.aid()).await; // 1 - 2
        let _ = nexus::add_edge::<FloodingPld>(node1.aid(), node3.aid()).await; // 1 - 3

        // STEP 3: Start the protocol
        let _ = node1.send(10.into(), FloodingPld::Start(12)).await;
    });
}
