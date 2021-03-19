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

    type private KMessage =
        | KRegister of IActorSink * AsyncReplyChannel<KSend>
        | KSend     of Packet list

    let make () =
        let mutable mbox : MailboxProcessor<KMessage> = Unchecked.defaultof<MailboxProcessor<KMessage>>

        let ksend aid (rs: RequestOut list) =            
            rs
            |> List.map (ofRequestOut aid)
            |> KSend
            |> mbox.Post
            |> ignore
            |> async.Return

        let fwd (actors: IActorSink list) (r: RequestIn) = 
            actors
            |> List.find (fun asink -> asink.Aid |> TID |> (=) r.Tid)
            |> fun asink -> asink.Received r

        let m = MailboxProcessor.Start(fun inbox ->

            let rec loop actors = async {
                match! inbox.Receive () with
                | KRegister (asink, rchnl) -> 
                    rchnl.Reply (ksend asink.Aid)
                    return! loop (asink::actors)
                | KSend pkts ->
                    pkts
                    |> List.map (toRequestIn >> fwd actors)
                    |> Async.ureduce
                    |> Async.RunSynchronously

                    return! loop actors
            }
            
            loop [])

        mbox <- m

        { new IKernel with
            member _.Register asink = 
                mbox.PostAndAsyncReply (fun rchnl -> KRegister (asink, rchnl))
        }