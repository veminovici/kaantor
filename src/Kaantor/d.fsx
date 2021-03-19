#load "Async.fs"
#load "Types.fs"
#load "Actor.fs"
#load "Kernel.fs"

open Simplee.Distributed

let krnl = Kernel.make ()

let a1 = Actor.spawn krnl (AID "a1")
let a2 = Actor.spawn krnl (AID "a2")

let r = { Tid = AID "a2"; Pld = PLD [|2uy; 0uy; 1uy|] }

a1.Post [r] |> Async.RunSynchronously
