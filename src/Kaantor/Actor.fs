namespace Simplee.Distributed

/// Implementation of the general actor functionality

[<RequireQualifiedAccessAttribute>]
module Actor =

    open Simplee

    /// The messages processed by an actor.
    type private AMessage =
        /// The actor received a message from another entity
        | AMsgReceive of DMessage

    /// Spawns a new actor using a given kernel. 
    /// The actor internal state is initialized using the passed in zero state.
    /// The actor uses the given handlers to process the incoming requests and the api calls.
    let make (krnl: IKernel) (rcv: DReceiveMessage<'TState>) (zro: 'TState) aid =

        let err  txt = Log.err  txt krnl
        let info txt = Log.info txt krnl

        let mutable ksend = Unchecked.defaultof<KernelSendFn>

        let mbox = MailboxProcessor.Start(fun inbox ->

            let rec loop stt = async {

                try
                    match! inbox.Receive() with
                    /// The actor received a message from the kernel.
                    | AMsgReceive msg -> 
                        let! msgs, stt' = rcv msg stt
                        do! ksend msgs
                        return! loop stt'
                with
                | e -> printfn "Error: %O" e }

            loop zro)

        /// Adds to the internal mailbox the incoming request.
        let postMessage msg =  
            msg 
            |> AMsgReceive 
            |> mbox.Post 
            |> async.Return

        /// The public actor's interface.
        let iActor = { new IActor with
            member _.Aid = aid }

        /// The internal actor sink, used by
        /// the kernel to send messages to this actor.
        let iActorSink = { new IActorSink with
            member _.Aid = aid
            member _.Post msg = postMessage msg }

        /// Registers the actor's sink with the kernel
        /// we get back a function which the actor will
        /// call whenever wants to send out requests.
        ksend <- 
            iActorSink
            |> krnl.Register
            |> Async.RunSynchronously

        /// Return the public interface and the function
        /// that helps the actor send the messages.
        iActor, iActorSink

