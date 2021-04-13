namespace Simplee.Distributed.Graph

open Simplee.Distributed

type Edge = {
    Nid: ActorId
    W:   float }

type INode =
    inherit IActor
    abstract member AddEdge: Edge -> Async<unit>
    abstract member Edges: Async<Edge list>

type NodeState = NodeState of Edge list
    with
    static member Empty = NodeState []

[<RequireQualifiedAccess>]
module Node =

    open Simplee
    open System.Threading.Tasks

    type private Msg =
        | MsgAddEdge of Edge
        | MsgEdges   of TaskCompletionSource<Edge list>

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
        let rcv (msg: DMessage) (NodeState edges) =
            match msg with
            | AddEdge edge -> 
                let msg, stt = [], edge :: edges |> NodeState
                (msg, stt) |> async.Return
            | Edges tcs -> 
                tcs.SetResult edges
                let msg, stt = [], (NodeState edges)
                (msg, stt) |> async.Return
            | _ -> 
                hmsg msg (NodeState edges)

        /// Create the actor, using the defined handlers, 
        /// and the initial empty list of log entries.
        let iActor, iActorSink, ksend = Actor.make krnl rcv NodeState.Empty aid

        let postMe pld = IActorSink.postMe aid pld iActorSink

        let postAddEdge edge = edge |> MsgAddEdge |> postMe
        let postEdges = let tcs = TaskCompletionSource<Edge list>() in tcs |> MsgEdges |> postMe |> Async.bind (fun _ -> tcs.Task |> Async.AwaitTask)

        /// The ILogger implementation.
        { new INode with 
            member _.Aid          = iActor.Aid
            member _.AddEdge edge = edge |> postAddEdge
            member _.Edges        = postEdges }