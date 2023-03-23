use crate::{SenderId, SessionId};
use actix::prelude::*;
use std::fmt::Debug;

pub struct ProtocolMsg<P> {
    sid: SenderId,
    kid: SessionId,
    pld: P,
}

impl<P: Debug> Debug for ProtocolMsg<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} | {:?} | {:?}", self.sid, self.kid, self.pld)
    }
}

impl<P> ProtocolMsg<P> {
    pub fn new(sid: SenderId, kid: SessionId, pld: P) -> Self {
        Self { kid, sid, pld }
    }

    pub fn payload(&self) -> &P {
        &self.pld
    }

    pub fn sid(&self) -> &SenderId {
        &self.sid
    }

    pub fn kid(&self) -> &SessionId {
        &self.kid
    }
}

impl<P> Message for ProtocolMsg<P> {
    type Result = ();
}
