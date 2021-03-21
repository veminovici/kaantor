namespace Simplee.Distributed.Graph

open Simplee.Distributed

type Neighbor = {
    Nid: ActorId
    W: float }

type INode =
    inherit IActor
    abstract member AddNeighbor: Neighbor -> unit
    abstract member Neighbors: Async<Neighbor list>

[<RequireQualifiedAccess>]
module Node =

    open Simplee

    /// The public apis
    type private NApi =
        | NApiAddNeighbor of Neighbor
        | NApiGetNeighbors

    let spawn (krnl: IKernel) aid =

        let hapi (args: obj) (ns: Neighbor list) =
            match (args :?> NApi) with
            | NApiAddNeighbor n -> () :> obj, [], n :: ns
            | NApiGetNeighbors  -> ns :> obj, [], ns

        let hmsg (r: RequestIn) (ns: Neighbor list) =
            [], ns

        let iActor, iActorInt = Actor.spawn krnl hapi hmsg [] aid

        let apiAddNeighbor n =
            n
            |> NApiAddNeighbor
            |> iActorInt.Api
            |> Async.RunSynchronously

        let apiNeighbors () = 
            NApiGetNeighbors
            |> iActorInt.Api 
            |> Async.map (fun o -> o :?> (Neighbor list))

        { new INode with 
            member _.Aid = iActor.Aid 
            member _.AddNeighbor n = n |> apiAddNeighbor |> ignore
            member _.Neighbors = apiNeighbors () }

    let addNeighbor n (node: INode) = node.AddNeighbor n
    let neighbors (node: INode) = node.Neighbors
    let aid (node: INode) = node.Aid 

