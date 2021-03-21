namespace Simplee.Distributed.Graph

open Simplee.Distributed

type NeighborLR = 
    | NeighborR of Neighbor
    | NeighborL of Neighbor

type INodeLR =
    inherit IActor
    abstract member AddNeighbor: NeighborLR -> unit
    abstract member Neighbors: Async<NeighborLR list>

[<RequireQualifiedAccess>]
module NodeR =

    open Simplee

    /// The public apis
    type private NApi =
        | NApiAddNeighbor of NeighborLR
        | NApiGetNeighbors

    let spawn (krnl: IKernel) aid =

        let hapi (args: obj) (ns: NeighborLR list) =
            match (args :?> NApi) with
            | NApiAddNeighbor n -> () :> obj, [], n :: ns
            | NApiGetNeighbors  -> ns :> obj, [], ns

        let hmsg (r: RequestIn) (ns: NeighborLR list) =
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
            |> Async.map (fun o -> o :?> (NeighborLR list))

        { new INodeLR with 
            member _.Aid = iActor.Aid 
            member _.AddNeighbor n = n |> apiAddNeighbor |> ignore
            member _.Neighbors = apiNeighbors () }

    let addNeighbor      n (node: INodeLR) = node.AddNeighbor n
    let addLeftNeighbor  n (node: INodeLR) = addNeighbor (NeighborL n) node
    let addRightNeighbor n (node: INodeLR) = addNeighbor (NeighborR n) node
    
    let neighbors (node: INodeLR) = node.Neighbors
    let aid (node: INodeLR) = node.Aid 

