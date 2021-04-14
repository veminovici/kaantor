namespace Simplee.Distributed

/// An actor is recording logging entries.

[<RequireQualifiedAccessAttribute>]
module Logger = 

    open Simplee
    open System.Threading.Tasks

    type private Stt = Stt of LEntry list
        with
        static member Empty = Stt []

    type private Msg =
        | MsgErr of string
        | MsgInfo of string
        | MsgLogs of TaskCompletionSource<LEntry list>

    let private (|AddErr|_|) msg =
        msg
        |> DMessage.pld
        |> function
        | :? Msg as msg ->
            match msg with
            | MsgErr txt -> Some txt
            | _ -> None
        | _ -> None

    let private (|AddInfo|_|) msg =
        msg
        |> DMessage.pld
        |> function
        | :? Msg as msg ->
            match msg with
            | MsgInfo txt -> Some txt
            | _ -> None
        | _ -> None

    let private (|GetLogs|_|) msg =
        msg
        |> DMessage.pld
        |> function
        | :? Msg as msg ->
            match msg with
            | MsgLogs tcs -> Some tcs
            | _ -> None
        | _ -> None

    /// Create a new logger actors using a given kernel
    let make (krnl: IKernel) =

        let aid = Sys.Ids.Logger

        /// Called when the logger receive a request.
        let rcv (msg: DMessage) (Stt logs) =
            let msgs, stt = 
                match msg with
                | AddInfo txt -> 
                    [], LInfo txt :: logs |> Stt
                | AddErr  txt -> 
                    [], LErr  txt :: logs |> Stt
                | GetLogs tcs -> 
                    tcs.SetResult logs
                    [], Stt logs
                | _ -> 
                    [], Stt logs

            (msgs, stt) |> async.Return

        /// Create the actor, using the defined handlers, 
        /// and the initial empty list of log entries.
        let iActor, iActorSink = Actor.make krnl rcv Stt.Empty aid

        let postMe = IActorSink.postMe iActorSink

        let postInfo s = s |> MsgInfo |> postMe
        let postErr  s = s |> MsgErr  |> postMe 
        let postLogs () = let tcs = TaskCompletionSource<LEntry list>() in tcs |> MsgLogs |> postMe |> Async.bind (fun _ -> tcs.Task |> Async.AwaitTask)

        /// The ILogger implementation.
        { new ILogger with 
            member _.Aid      = iActor.Aid 
            member _.Info msg = postInfo msg
            member _.Err msg  = postErr msg
            member _.Logs     = postLogs () }

    /// Add new error log entry
    let err msg (l: ILogger) = l.Err msg

    /// Add new info log entry
    let info msg (l: ILogger) = l.Info msg

    let logs (l: ILogger) = l.Logs