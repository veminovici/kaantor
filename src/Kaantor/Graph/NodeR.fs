namespace Simplee.Distributed.Graph

open Simplee.Distributed

type EdgeR = 
    | EdgeRight of Edge
    | EdgeLeft  of Edge

type INodeR =
    inherit IActor
    abstract member AddEdge: EdgeR -> Async<unit>
    abstract member Edges: Async<EdgeR list>

type NodeRState = NodeRState of EdgeR list
    with
    static member Empty = NodeRState []

[<RequireQualifiedAccess>]
module NodeR =

    open Simplee
    open System.Threading.Tasks

    type private Msg =
        | MsgAddEdge of EdgeR
        | MsgEdges   of TaskCompletionSource<EdgeR list>

    let private (|AddEdge|_|) msg =
        msg
        |> DMessage.pld
        |> function
        | :? Msg as msg ->
            match msg with
            | MsgAddEdge edge -> Some edge
            | _ -> None
        | _ -> None

    let private (|Edges|_|) msg =
        msg
        |> DMessage.pld
        |> function
        | :? Msg as msg ->
            match msg with
            | MsgEdges tcs -> Some tcs
            | _ -> None
        | _ -> None

    let make (krnl: IKernel) hmsg aid =

        /// Called when the logger receive a request.
        let rcv (msg: DMessage) (NodeRState edges) =
            match msg with
            | AddEdge edge -> 
                let msgs, stt = [], edge :: edges |> NodeRState
                (msgs, stt) |> async.Return
            | Edges tcs -> 
                tcs.SetResult edges
                let msgs, stt = [], (NodeRState edges)
                (msgs, stt) |> async.Return
            | _ -> 
                hmsg msg (NodeRState edges)

        /// Create the actor, using the defined handlers, 
        /// and the initial empty list of log entries.
        let iActor, iActorSink = Actor.make krnl rcv NodeRState.Empty aid

        let postMe = IActorSink.postMe iActorSink

        let postAddEdge edge = edge |> MsgAddEdge |> postMe
        let postEdges () = let tcs = TaskCompletionSource<EdgeR list>() in tcs |> MsgEdges |> postMe |> Async.bind (fun _ -> tcs.Task |> Async.AwaitTask)

        /// The ILogger implementation.
        { new INodeR with 
            member _.Aid          = iActor.Aid
            member _.AddEdge edge = edge |> postAddEdge
            member _.Edges        = postEdges () }