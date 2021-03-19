namespace Simplee.Distributed

[<RequireQualifiedAccessAttribute>]
module Actor = 
    type private AMessage =
        | AReceived  of RequestIn
        | APublicAPI of Payload

    let spawn (krnl: IKernel) (aid: ActorId) =
    
        let mbox = MailboxProcessor.Start(fun inbox ->

            let rec loop () = async {
                match! inbox.Receive() with
                | AReceived req -> 
                    printfn "We received a request: %O" req
                    return! loop ()
                | APublicAPI pld ->
                    printfn "We received a public API %O" pld
                    return! loop ()
            }
            
            loop ())

        let postReceived req = async{ mbox.Post (AReceived req) }

        let ksend = 
            { new IActorSink with
                member _.Aid = aid
                member _.Received req = postReceived req 
            }
            |> krnl.Register
            |> Async.RunSynchronously

        let iActor = { new IActor with
            member _.Aid = async {return aid}
        }

        let iActorInt = { new IActorInt with
            member _.SendRequests rs = ksend rs 
            member _.CallPublicApi pld = mbox.Post (APublicAPI pld)
        }

        iActor, iActorInt