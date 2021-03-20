namespace Simplee.Distributed

/// Implementation of the general actor functionality

[<RequireQualifiedAccessAttribute>]
module Actor =

    /// The messages processed by an actor.
    type private AMessage =
        /// The actor received a request from another entity
        | AMsgReceived of RequestIn
        /// The actor received an api call from an external user.
        | AMsgApi of obj * AsyncReplyChannel<obj>

    /// Spawns a new actor using a given kernel. 
    /// The actor internal state is initialized using the passed in zero state.
    /// The actor uses the given handlers to process the incoming requests and the api calls.
    let spawn (krnl: IKernel) (hapi: ActorApiHndl<'a>) (hmsg: ActorMsgHndl<'a>) zro aid =

        let mutable ksend = Unchecked.defaultof<KernelSendFn>

        let mbox = MailboxProcessor.Start(fun inbox ->

            let rec loop stt = async {

                try
                match! inbox.Receive() with

                /// Another actor sent us a request
                | AMsgReceived req -> 
                    let reqs, stt' = hmsg req stt
                    return! loop stt'

                /// A caller invoked a public api.
                | AMsgApi (args, rchnl) ->
                    let res, rs, stt' = hapi args stt

                    rchnl.Reply res

                    // send the requests out.
                    do! ksend rs

                    return! loop stt'
                with
                | e -> printfn "Error: %O" e
            }

            loop zro)

        /// Adds to the internal mailbox the incoming request.
        let postReceived req = async{ mbox.Post (AMsgReceived req) }

        /// Registers the actor's sink with the kernel
        /// we get back a function which the actor will
        /// call whenever wants to send out requests.
        ksend <- 
            { new IActorSink with
                member _.Aid = aid
                member _.Received req = postReceived req 
            }
            |> krnl.Register
            |> Async.RunSynchronously

        /// The public actor's interface.
        let iActor = { new IActor with
            member _.Aid = aid }

        /// The internal actor's interfaces.
        /// This interface will be used to the more specific
        /// actors to send requests to other actors.
        let iActorInt = { new IActorInt with
            member _.SendRequests rs = ksend rs 
            member _.Api        args = mbox.PostAndAsyncReply (fun r -> AMsgApi (args, r)) }

        /// Return the public and internal 
        /// actor's interfaces
        iActor, iActorInt

