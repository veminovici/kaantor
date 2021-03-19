namespace Simplee.Distributed

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

        let hmsg (r: RequestIn) (logs: LEntry list) =
            [], logs

        let iActor, iActorInt = Actor.spawn krnl hapi hmsg [] (AID "sys:logger")

        let apiErr msg = 
            msg 
            |> LErr 
            |> LApiAddEntry 
            |> iActorInt.Api 
            |> Async.RunSynchronously

        let apiInfo msg =
            msg
            |> LInfo
            |> LApiAddEntry
            |> iActorInt.Api
            |> Async.RunSynchronously

        let apiLogs () = 
            LApiGetLogs 
            |> iActorInt.Api 
            |> Async.map (fun o -> o :?> (LEntry list))

        { new ILogger with 
            member _.Aid      = iActor.Aid 
            member _.Info msg = msg |> apiInfo |> ignore
            member _.Err msg  = msg |> apiErr  |> ignore 
            member _.Logs     = apiLogs () }

    let err  (l: ILogger) msg = l.Err  msg
    let info (l: ILogger) msg = l.Info msg
    let logs (l: ILogger)     = l.Logs