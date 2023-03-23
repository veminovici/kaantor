use std::fmt::Debug;

/// The unique identifier for a session (aka correlation id).
#[derive(Clone, Copy)]
pub struct SessionId(usize);

impl Debug for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "K{}", self.0)
    }
}

impl From<usize> for SessionId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
