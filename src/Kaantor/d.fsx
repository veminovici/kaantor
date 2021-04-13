//#r "nuget: FsPickler"

#load "Async.fs"
#load "Types.fs"
#load "Actor.fs"
#load "Logger.fs"
#load "Kernel.fs"

open Simplee
open Simplee.Distributed

printfn "Starting testing"

let krnl = Kernel.make ()

krnl |> Kernel.err "test1" |> Async.RunSynchronously
krnl |> Kernel.err "test2" |> Async.RunSynchronously
krnl |> Kernel.err "test3" |> Async.RunSynchronously
krnl |> Kernel.err "test4" |> Async.RunSynchronously
krnl |> Kernel.err "test5" |> Async.RunSynchronously
krnl |> Kernel.logs |> Async.RunSynchronously |> printfn "Logs: %O"

printfn "Ended testing"
