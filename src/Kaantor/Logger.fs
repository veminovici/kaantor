namespace Simplee.Distributed

/// An actor is recording logging entries.

[<RequireQualifiedAccessAttribute>]
module Logger = 

    open Simplee

    /// The public api exposed by the Logger actor
    type private LApi =
        /// Add a new log entry
        | LApiAddEntry of LEntry
        /// Returns the log entries
        | LApiGetLogs

    /// Create a new logger actors using a given kernel
    let spawn (krnl: IKernel) =

        /// Called when a new public api call is placed by an external caller
        let hapi (args: obj) (logs: LEntry list) =
            match (args :?> LApi) with

            // Add a new log entry to the internal list of entries
            | LApiAddEntry l -> () :> obj, [], l :: logs

            // Returns the internal list of entries.
            | LApiGetLogs -> logs :> obj, [], logs

        /// Called when the logger receive a request.
        let hmsg (r: RequestIn) (logs: LEntry list) =
            [], logs

        /// Create the actor, using the defined handlers, 
        /// and the initial empty list of log entries.
        let iActor, iActorInt = Actor.spawn krnl hapi hmsg [] (AID "sys:logger")

        /// Helper function to process the public api call
        /// which adds a new error log entry.
        let apiErr msg = 
            msg 
            |> LErr 
            |> LApiAddEntry 
            |> iActorInt.Api 
            |> Async.RunSynchronously

        /// Helper function to process the public api call
        /// which adds a new info log entry.
        let apiInfo msg =
            msg
            |> LInfo
            |> LApiAddEntry
            |> iActorInt.Api
            |> Async.RunSynchronously

        /// Helper function to process the public api call
        /// which should return the current list of log entries.
        let apiLogs () = 
            LApiGetLogs 
            |> iActorInt.Api 
            |> Async.map (fun o -> o :?> (LEntry list))

        /// The ILogger implementation.
        { new ILogger with 
            member _.Aid      = iActor.Aid 
            member _.Info msg = msg |> apiInfo |> ignore
            member _.Err msg  = msg |> apiErr  |> ignore 
            member _.Logs     = apiLogs () }

    /// Add new error log entry
    let err  (l: ILogger) msg = l.Err  msg

    /// Add new info log entry
    let info (l: ILogger) msg = l.Info msg

    /// Return the lost of log entries.
    let logs (l: ILogger)     = l.Logs