namespace Simplee.Distributed

[<RequireQualifiedAccessAttribute>]
module Actor = 
    type private AMessage =
        | AReceived  of RequestIn
        | APublicAPI of Payload

    let spawn (krnl: IKernel) hApi zro (aid: ActorId) =
    
        let mbox = MailboxProcessor.Start(fun inbox ->

            let rec loop stt = async {
                match! inbox.Receive() with
                | AReceived req -> 
                    printfn "We received a request: %O" req
                    return! loop stt
                | APublicAPI pld ->
                    let stt' = hApi stt pld
                    return! loop stt'
            }
            
            loop zro)

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
            member _.CallPublicApi pld = 
                mbox.Post (APublicAPI pld)
        }

        iActor, iActorInt