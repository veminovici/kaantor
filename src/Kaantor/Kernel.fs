namespace Simplee.Distributed

open Simplee

/// Implementation of the kernel

[<RequireQualifiedAccessAttribute>]
module Kernel = 

    /// The internal packet sent over inside the kernel infrastructure
    type private Packet = {
        Hdr: Header
        Pld: Payload }

    /// Converts outgoing requests to an internal packet.
    let private ofRequestOut aid (r: RequestOut) = 
        let hdr = {Fid = FID aid; Tid = TID r.Tid} in { Hdr = hdr; Pld = r.Pld }

    /// Converts an internal packet to an incoming request.
    let private toRequestIn (p: Packet) : RequestIn = 
        { Hdr = p.Hdr; Pld = p.Pld }

    /// The messages handled by the kernel mailbox.
    type private KMessage =

        /// Register a new actor.
        | KMsgRegister of IActorSink * AsyncReplyChannel<KernelSendFn>

        /// Send a list of packets.
        | KMsgSend     of Packet list

    /// Create a new kernel.
    let make () =

        // The mailbox used by the kernel system. Use have this in order to refer the mailbox
        // in our functions before it is really created.
        let mutable mbox : MailboxProcessor<KMessage> = Unchecked.defaultof<MailboxProcessor<KMessage>>
        let mutable lgr : ILogger = Unchecked.defaultof<ILogger>

        // receives the requests sent by a client
        // converts the requests to the internal representation, packets,
        // and puts these packets in the internal queue to be processed.
        let ksend aid (rs: RequestOut list) =
            rs
            |> List.map (ofRequestOut aid)
            |> KMsgSend
            |> mbox.Post
            |> ignore
            |> async.Return

        // forwards a request to a given actor.
        let fwd (actors: IActorSink list) (r: RequestIn) = 
            actors
            |> List.find (fun asink -> asink.Aid |> TID |> (=) r.Hdr.Tid)
            |> fun asink -> asink.Received r

        // create the mailbox.
        mbox <- MailboxProcessor.Start(fun inbox ->

            let rec loop actors = async {
                match! inbox.Receive () with

                // Message where we need to register a give actor.
                | KMsgRegister (asink, rchnl) ->
                    rchnl.Reply (ksend asink.Aid)
                    return! loop (asink::actors)

                // Message where the need to dispatch
                // a list of packets.
                | KMsgSend pkts ->
                    pkts
                    |> List.map (toRequestIn >> fwd actors)
                    |> Async.reduceU
                    |> Async.RunSynchronously

                    return! loop actors
            }
            
            loop [])

        /// Create a kernel implementation, without a logger.
        let krnl = 
            { new IKernel with
                member _.Register asink = mbox.PostAndAsyncReply (fun rchnl -> KMsgRegister (asink, rchnl))
                member _.Logger = lgr }

        /// Create the logger, using the newly created kernel
        lgr <- Logger.spawn krnl

        /// Recreate the kernel implementation, this time using the logger.
        { new IKernel with
            member _.Register asink = mbox.PostAndAsyncReply (fun rchnl -> KMsgRegister (asink, rchnl))
            member _.Logger = lgr }