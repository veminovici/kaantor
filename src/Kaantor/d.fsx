//#r "nuget: FsPickler"

#load "Async.fs"
#load "Types.fs"
#load "Actor.fs"
#load "Logger.fs"
#load "Kernel.fs"
#load "Rebounder.fs"
#load "Pinger.fs"

open Simplee
open Simplee.Distributed

printfn "Starting testing"

let krnl = Kernel.make ()

let testLogging () =
    krnl |> Kernel.err "test1" |> Async.RunSynchronously
    krnl |> Kernel.err "test2" |> Async.RunSynchronously
    krnl |> Kernel.err "test3" |> Async.RunSynchronously
    krnl |> Kernel.err "test4" |> Async.RunSynchronously
    krnl |> Kernel.err "test5" |> Async.RunSynchronously
    krnl |> Kernel.logs |> Async.RunSynchronously |> printfn "Logs: %O"

let testPingPong () =
    let ping1 = Pinger.make krnl (AID "ping1")
    let ping2 = Pinger.make krnl (AID "ping2")

    ping1.Ping (AID "ping2") (SID "mySession") |> Async.RunSynchronously |> printfn "Ping-pong: %O"
    krnl |> Kernel.logs |> Async.RunSynchronously |> printfn "Logs: %O"

//testLogging()
testPingPong()

printfn "Ended testing"
