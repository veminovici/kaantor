namespace Simplee.Distributed

type IRebounder =
    inherit IActor
    abstract member Rebounds: Async<int>

[<RequireQualifiedAccess>]
module Rebounder =

    open Simplee
    open System.Threading.Tasks

    type private RebounderState = RebounderState of int
        with
        static member Empty = RebounderState 0

    let make (krnl: IKernel) hmsg aid =

        /// Called when the logger receive a request.
        let rcv (msg: DMessage) (RebounderState rs) =
            let msgs, stt = [], (RebounderState (rs + 1))
            (msgs, stt) |> async.Return

        /// Create the actor, using the defined handlers, 
        /// and the initial empty list of log entries.
        let iActor, _ = Actor.make krnl rcv RebounderState.Empty aid

        /// The ILogger implementation.
        iActor