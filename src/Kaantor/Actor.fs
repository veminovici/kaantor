namespace Simplee.Distributed

[<RequireQualifiedAccessAttribute>]
module Actor =

    type private AMessage =
        | AMsgReceived of RequestIn
        | AMsgApi of obj * AsyncReplyChannel<obj>

    let spawn (krnl: IKernel) (hapi: obj -> 'a -> obj * 'a) zro aid =

        let mbox = MailboxProcessor.Start(fun inbox ->
            
            let rec loop stt = async {

                try
                match! inbox.Receive() with

                /// Another actor sent us a request
                | AMsgReceived req -> 
                    printfn "We received a request: %O" req
                    return! loop stt

                /// A caller invoked a public api.
                | AMsgApi (args, rchnl) ->
                    let res, stt' = hapi args stt
                    rchnl.Reply res// VLD - here we need to put the returns value.
                    return! loop stt'
                with
                | e -> printfn "Error: %O" e
            }

            loop zro)

        let postReceived req = async{ mbox.Post (AMsgReceived req) }

        let ksend = 
            { new IActorSink with
                member _.Aid = aid
                member _.Received req = postReceived req 
            }
            |> krnl.Register
            |> Async.RunSynchronously

        let iActor = { new IActor with
            member _.Aid      = aid }

        let iActorInt = { new IActorInt with
            member _.SendRequests rs   = ksend rs 
            member _.Api args = mbox.PostAndAsyncReply (fun r -> AMsgApi (args, r)) }

        iActor, iActorInt

