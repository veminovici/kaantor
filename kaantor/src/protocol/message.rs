use crate::{ActorId, IntoActorId, SenderId, SessionId};
use actix::prelude::*;
use log::info;
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

    pub fn deconstruct_rcvd<A>(&self, actor: &A, sfx: &str) -> (ActorId, SenderId, SessionId, P)
    where
        A: IntoActorId,
        P: Copy + Debug,
    {
        let me = actor.aid();
        let kid = *self.kid();
        let sid = *self.sid();
        let pld = *self.payload();

        info!(
            "{:?} || RCVD | {:?} >> {:?} | {:?} | {:?} | {:?}",
            &me, &sid, &me, &kid, &pld, sfx
        );

        (me, sid, kid, pld)
    }
}

impl<P> Message for ProtocolMsg<P> {
    type Result = ();
}
