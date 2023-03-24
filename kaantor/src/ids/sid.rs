use crate::ActorId;
use std::fmt::Debug;

/// It represents the identifier for the sender of the message.
#[derive(Clone, Copy, Default)]
pub struct SenderId(ActorId);

impl SenderId {
    pub fn aid(&self) -> ActorId {
        self.0
    }
}

impl Debug for SenderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<ActorId> for SenderId {
    fn from(value: ActorId) -> Self {
        Self(value)
    }
}
