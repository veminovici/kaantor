mod ids;
pub mod nexus;
mod protocol;

pub use ids::*;
pub use protocol::*;

pub trait IntoActorId {
    fn aid(&self) -> ActorId;
}
