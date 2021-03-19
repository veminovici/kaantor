namespace Simplee.Distributed

type LEntry =
    | LErr of string

type ILogger =
    inherit IActor
    abstract member Err: string -> unit
    abstract member Logs: Async<LEntry list>

[<RequireQualifiedAccessAttribute>]
module Logger = 

    open Simplee

    /// The public apis
    type private LApi =
        | LApiAddEntry of LEntry
        | LApiGetLogs

    let spawn (krnl: IKernel) =

        let hapi (args: obj) (logs: LEntry list) =
            match (args :?> LApi) with
            | LApiAddEntry l -> () :> obj, l :: logs
            | LApiGetLogs -> logs :> obj, logs

        let iActor, iActorInt = Actor.spawn krnl hapi [] (AID "sys:logger")

        let apiErr msg = 
            msg 
            |> LErr 
            |> LApiAddEntry 
            |> iActorInt.Api 
            |> Async.RunSynchronously

        let apiLogs () = 
            LApiGetLogs 
            |> iActorInt.Api 
            |> Async.map (fun o -> o :?> (LEntry list))

        { new ILogger with 
            member _.Aid      = iActor.Aid 
            member _.Err msg  = msg |> apiErr |> ignore 
            member _.Logs     = apiLogs () }
