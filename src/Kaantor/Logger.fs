namespace Simplee.Distributed

type LEntry =
    | LErr  of string
    | LInfo of string
    | LReq  of string

type ILoggerActor =
    inherit IActor

    abstract member Err: string  -> unit
    abstract member Info: string -> unit
    abstract member Req: string  -> unit

[<RequireQualifiedAccessAttribute>]
module Logger =

    let spawn (kernel: IKernel) =
        let iActor, iActorInt = Actor.spawn kernel (AID "sys:logger")

        let iLogger = { new ILoggerActor with
            member _.Aid = iActor.Aid 
            member _.Err  msg = ()
            member _.Info msg = ()
            member _.Req  msg = ()
        }

        iLogger, iActorInt