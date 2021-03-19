#load "Async.fs"
#load "Types.fs"
#load "Actor.fs"
#load "Logger.fs"
#load "Kernel.fs"

open Simplee.Distributed

let krnl = Kernel.make () 

let a1, a1Int = Kernel.spawn (AID "a1") krnl
let a2, a2Int = Kernel.spawn (AID "a2") krnl

let r = { Tid = AID "a2"; Pld = PLD [|2uy; 0uy; 1uy|] }

a1Int.SendRequests [r] |> Async.RunSynchronously
