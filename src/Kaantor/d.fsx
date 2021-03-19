//#r "nuget: FsPickler"

#load "Async.fs"
#load "Types.fs"
#load "Actor.fs"
#load "Logger.fs"
#load "Kernel.fs"

open Simplee.Distributed

printfn "Starting testing"

let krnl = Kernel.make ()
let log = Logger.spawn krnl

[1..5]
|> List.map (sprintf "msg%d")
|> List.iter log.Err

log.Logs |> Async.RunSynchronously |> printfn "Logs: %O"

printfn "Ended testing"
