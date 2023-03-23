mod ids;
pub mod nexus;
mod protocol;

pub use ids::*;
pub use protocol::*;

pub trait KActor {
    fn aid(&self) -> &ActorId;
}
