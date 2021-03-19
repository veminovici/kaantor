namespace Simplee.Distributed

open Simplee

[<RequireQualifiedAccessAttribute>]
module Kernel = 

    type private Packet = {
        Fid: FromId
        Tid: ToId
        Pld: Payload }

    let private ofRequestOut aid (r: RequestOut) = { 
        Fid = FID aid
        Tid = TID r.Tid
        Pld = r.Pld }

    let private toRequestIn (p: Packet) : RequestIn = {
        Fid = p.Fid
        Tid = p.Tid
        Pld = p.Pld }

    /// The messages handled by the kernel mailbox.
    type private KMessage =
        | KRegister of IActorSink * AsyncReplyChannel<KSend>
        | KSend     of Packet list

    let make () =

        // The mailbox used by the kernel system. Use have this in order to refer the mailbox
        // in our functions before it is really created.
        let mutable mbox : MailboxProcessor<KMessage> = Unchecked.defaultof<MailboxProcessor<KMessage>>
        let mutable lgr : ILoggerActor = Unchecked.defaultof<ILoggerActor>

        // receives the requests sent by a client
        // converts the requests to the internal representation, packets,
        // and puts these packets in the internal queue to be processed.
        let ksend aid (rs: RequestOut list) =
            rs
            |> List.map (ofRequestOut aid)
            |> KSend
            |> mbox.Post
            |> ignore
            |> async.Return

        // forwards a request to a given actor.
        let fwd (actors: IActorSink list) (r: RequestIn) = 
            actors
            |> List.find (fun asink -> asink.Aid |> TID |> (=) r.Tid)
            |> fun asink -> asink.Received r

        // create the mailbox.
        mbox <- MailboxProcessor.Start(fun inbox ->

            let rec loop actors = async {
                match! inbox.Receive () with

                // Message where we need to register a give actor.
                | KRegister (asink, rchnl) ->
                    rchnl.Reply (ksend asink.Aid)
                    return! loop (asink::actors)

                // Message where the need to dispatch
                // a list of packets.
                | KSend pkts ->
                    lgr.Info "Dispatching some packets ..."

                    pkts
                    |> List.map (toRequestIn >> fwd actors)
                    |> Async.ureduce
                    |> Async.RunSynchronously

                    return! loop actors
            }
            
            loop [])

        let krnl = 
            { new IKernel with
                member _.Register asink = 
                    mbox.PostAndAsyncReply (fun rchnl -> KRegister (asink, rchnl))
            }

        let lgrAPI, lgrInt = Logger.spawn krnl
        lgr <- lgrAPI

        krnl

    let spawn aid hApi zro kernel = 
        Actor.spawn kernel hApi zro aid