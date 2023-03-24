use crate::{ActorId, ProtocolMsg};
use actix::prelude::*;
use log::debug;
use std::fmt::Debug;

pub(crate) struct ProtocolPxy<P: Send> {
    aid: ActorId,
    recipient: Recipient<ProtocolMsg<P>>,
}

impl<P: Send> Debug for ProtocolPxy<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.aid)
    }
}

impl<P: Send> ProtocolPxy<P> {
    /// Returns the actor identifier
    pub(crate) fn aid(&self) -> &ActorId {
        &self.aid
    }

    /// Creates a new proxy for a given actor and its recipient
    pub(crate) fn new(aid: ActorId, recipient: Recipient<ProtocolMsg<P>>) -> Self {
        Self { aid, recipient }
    }

    /// Logs the debug proxy information
    fn debug_msg(&self, pfx: &str, msg: &ProtocolMsg<P>)
    where
        P: Debug,
    {
        debug!(
            "APXY:{:04?} || {} | {:?} -> {:?} | {:?} |> {:?}",
            self.aid,
            pfx,
            msg.sid(),
            self.aid,
            msg.kid(),
            msg.payload()
        );
    }

    pub fn try_send(
        &self,
        msg: ProtocolMsg<P>,
    ) -> Result<<ProtocolMsg<P> as Message>::Result, SendError<ProtocolMsg<P>>>
    where
        P: Debug,
    {
        self.debug_msg("PRXY", &msg);
        self.recipient.try_send(msg)
    }

    // pub async fn send(
    //     &self,
    //     msg: ProtocolMsg<P>,
    // ) -> Result<<ProtocolMsg<P> as Message>::Result, MailboxError>
    // where
    //     P: Debug,
    // {
    //     self.debug_msg("ASND", &msg);
    //     self.recipient.send(msg).await
    // }
}
