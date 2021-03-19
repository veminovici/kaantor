namespace Simplee.Distributed

[<RequireQualifiedAccessAttribute>]
module Actor = 
    type private AMessage =
        | AReceived of RequestIn

    let spawn (krnl: IKernel) (aid: ActorId) =
        let mbox = MailboxProcessor.Start(fun inbox ->

            let rec loop () = async {
                match! inbox.Receive() with
                | AReceived req -> 
                    printfn "We received a request"
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

        { new IActor with
            member _.Aid = async {return aid}
            member _.Post rs = ksend rs 
        }