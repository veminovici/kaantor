namespace Simplee.Distributed

type private LEntry =
    | LErr  of string
    | LInfo of string
    | LReq  of string

type ILoggerActor =
    inherit IActor

    abstract member Err:  string -> unit
    abstract member Info: string -> unit
    abstract member Req:  string -> unit

[<RequireQualifiedAccessAttribute>]
module Logger =

    open MBrace.FsPickler

    let spawn (kernel: IKernel) =
        let bser = FsPickler.CreateBinarySerializer()
        let zro = []

        let hApi stt (PLD pld) = 
            let lentry = bser.UnPickle<LEntry> pld
            printfn "Logger entry: %O" lentry
            lentry :: stt

        let iActor, iActorInt = Actor.spawn kernel hApi zro (AID "sys:logger")

        let iLogger = { new ILoggerActor with
            member _.Aid = iActor.Aid 
            member _.Err  msg = msg |> LErr  |> bser.Pickle |> PLD |> iActorInt.CallPublicApi
            member _.Info msg = msg |> LInfo |> bser.Pickle |> PLD |> iActorInt.CallPublicApi
            member _.Req  msg = msg |> LReq  |> bser.Pickle |> PLD |> iActorInt.CallPublicApi
        }

        iLogger, iActorInt