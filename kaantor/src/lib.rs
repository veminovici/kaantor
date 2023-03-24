mod ids;
pub mod nexus;
pub mod node;
mod protocol;

pub use ids::*;
pub use node::*;
pub use protocol::*;

pub trait IntoActorId {
    fn aid(&self) -> ActorId;
}
