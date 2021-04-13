namespace Simplee.Distributed

open Simplee

/// Implementation of the kernel

[<RequireQualifiedAccessAttribute>]
module Kernel = 

    open System.Threading.Tasks

    type Stt = Stt of IActorSink list
        with
        static member Empty = Stt []

    type Msg =
        | MsgRegister of IActorSink * TaskCompletionSource<KernelSendFn>
        | MsgDeliver of DMessage

    /// Create a new kernel.
    let make () =

        // The mailbox used by the kernel system. Use have this in order to refer the mailbox
        // in our functions before it is really created.
        let mutable mbox : MailboxProcessor<Msg> = Unchecked.defaultof<MailboxProcessor<Msg>>
        let mutable lgr : ILogger = Unchecked.defaultof<ILogger>

        let postDeliver msg = msg |> MsgDeliver |> mbox.Post

        // create the mailbox.
        mbox <- MailboxProcessor.Start(fun inbox ->

            let rec loop (Stt sinks) = async {
                match! inbox.Receive () with
                | MsgRegister (sink, tcs) ->
                    let sinks = sink :: sinks
                    let ksend msgs = 
                        msgs
                        |> List.map postDeliver
                        |> ignore
                        |> async.Return
                    tcs.SetResult ksend
                    return! loop (Stt sinks)
                | MsgDeliver msg ->
                    return! 
                        sinks
                        |> List.find (fun snk -> (TID snk.Aid) = msg.Hdr.Tid)
                        |> fun snk -> snk.Post msg
                        |> Async.bind (fun _ -> loop (Stt sinks)) }
            
            loop Stt.Empty)

        let register sink =
            let tcs = TaskCompletionSource<KernelSendFn>()
            (sink, tcs) |> MsgRegister |> mbox.Post
            tcs.Task |> Async.AwaitTask

        /// Create the logger, using the newly created kernel
        lgr <- Logger.make { new IKernel with
                member _.Register sink = register sink
                member _.Logger = lgr }

        /// Recreate the kernel implementation, this time using the logger.
        { new IKernel with
            member _.Register sink = register sink
            member _.Logger = lgr }

    //
    // Logging functions
    //

    let private logger (k: IKernel) = k.Logger
    let err  msg (k: IKernel) = k |> logger |> Logger.err msg
    let info msg (k: IKernel) = k |> logger |> Logger.info msg
    let logs     (k: IKernel) = k |> logger |> Logger.logs