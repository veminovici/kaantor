//#r "nuget: FsPickler"

#load "Async.fs"
#load "Types.fs"
#load "Actor.fs"
#load "Logger.fs"
#load "Kernel.fs"

open Simplee.Distributed

printfn "Starting testing"


let krnl = Kernel.make ()

krnl |> Kernel.logger |> Logger.err "testing" |> Async.RunSynchronously
krnl |> Kernel.logger |> Logger.logs |> Async.RunSynchronously |> printfn "Logs: %O"

printfn "Ended testing"
