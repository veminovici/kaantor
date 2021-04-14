namespace Simplee.Distributed

type IPinger =
    inherit IActor
    abstract member Ping: ActorId -> SessionId -> Async<SessionId>

[<RequireQualifiedAccess>]
module Pinger =

    open Simplee
    open System.Threading.Tasks

    type PingerState = PingerState of (SessionId * TaskCompletionSource<SessionId>) list
        with
        static member Empty = PingerState []

    type private Msg =
        | MsgStart of ToId * SessionId * TaskCompletionSource<SessionId>
        | MsgPing  of SessionId
        | MsgPong  of SessionId

    let private (|Start|_|) msg =
        msg
        |> DMessage.pld
        |> function
        | :? Msg as msg ->
            match msg with
            | MsgStart (tid, sid, tcs) -> Some (tid, sid, tcs)
            | _ -> None
        | _ -> None

    let private (|Ping|_|) msg =
        msg
        |> DMessage.pld
        |> function
        | :? Msg as msg ->
            match msg with
            | MsgPing sid -> Some sid
            | _ -> None
        | _ -> None

    let private (|Pong|_|) msg =
        msg
        |> DMessage.pld
        |> function
        | :? Msg as msg ->
            match msg with
            | MsgPong sid -> Some sid
            | _ -> None
        | _ -> None

    let make (krnl: IKernel) aid =

        /// Called when the logger receive a request.
        let rcv (msg: DMessage) (PingerState xs) =
            let msgs, stt = 
                match msg with
                | Start (tid, sid, tcs) ->
                    let msgs = 
                        DMessage.Empty 
                        |> DMessage.withFrom (FID aid)
                        |> DMessage.withTo   tid
                        |> DMessage.withPld  (MsgPing sid)
                        |> List.singleton

                    msgs, PingerState ((sid, tcs) :: xs)

                | Ping sid ->
                    let msgs =
                        DMessage.Empty
                        |> DMessage.withHdr (Header.flip msg.Hdr)
                        |> DMessage.withPld (MsgPong sid)
                        |> List.singleton

                    msgs, PingerState xs

                | Pong sid ->

                    let folder (found, xs) (s, tcs) =
                        if s = sid then Some (s, tcs), xs
                        else found, (s, tcs) :: xs

                    xs
                    |> List.fold folder (None, [])
                    |> function
                    | (None, _) -> 
                        [], PingerState xs
                    | Some (s, tcs), xs -> 
                        tcs.SetResult(s)
                        [], PingerState (List.rev xs)

                | _ -> 
                    [], PingerState xs

            (msgs, stt) |> async.Return

        /// Create the actor, using the defined handlers, 
        /// and the initial empty list of log entries.
        let iActor, iActorSink = Actor.make krnl rcv PingerState.Empty aid

        let postMe = IActorSink.postMe iActorSink

        let postStart tid sid = let tcs = TaskCompletionSource<SessionId>() in (TID tid, sid, tcs) |> MsgStart |> postMe |> Async.bind (fun _ -> tcs.Task |> Async.AwaitTask)

        /// The ILogger implementation.
        { new IPinger with 
            member _.Aid          = iActor.Aid
            member _.Ping aid sid = postStart aid sid }