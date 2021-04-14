namespace Simplee.Distributed

/// The actor identifier.
type ActorId = AID of string with
    static member Empty = AID ""
    override this.ToString() = let (AID i) = this in i

/// The identifier of the source of the message
type FromId  = FID of ActorId with
    static member Empty = FID ActorId.Empty
    override this.ToString() = let (FID (AID i)) = this in i

/// The identifier of the destination of the message
type ToId    = TID of ActorId with
    static member Empty = TID ActorId.Empty
    override this.ToString() = let (TID (AID i)) = this in i

type SessionId = SID of string with
    static member Empty = SID ""
    override this.ToString() = let (SID i) = this in i

/// The header of the message
type Header = {
    Fid: FromId
    Tid: ToId } with
    static member Empty = { Fid = FromId.Empty; Tid = ToId.Empty }
    override this.ToString() = sprintf "%O->%O" this.Fid this.Tid

/// The payload of the message. The payload is just an array of bytes.
type Payload = PLD of obj with
    static member Empty = PLD null
    override this.ToString() = let (PLD o) = this in sprintf "%A" o

/// A received message, with a given payload, from an actor.
type DMessage = {
    Hdr: Header
    Pld: Payload } with
    static member Empty = { Hdr = Header.Empty; Pld = Payload.Empty }

/// The behavior of an actor. It just needs to return its identity.
type IActor =
    abstract member Aid:  ActorId

/// The actor sink behavior: can return the actor identifier, or handle an incoming message.
type IActorSink =
    abstract member Aid:  ActorId
    abstract member Post: DMessage -> Async<unit>

/// The log entry.
type LEntry =
    | LErr  of string
    | LInfo of string

/// The behavior of the logger actor. It inherits the actor
/// general behavior. It also can log an info, or an err, and
/// it can return the recorded entries.
type ILogger =
    inherit IActor
    abstract member Info: string -> Async<unit>
    abstract member Err:  string -> Async<unit>
    abstract member Logs: Async<LEntry list>

/// An alias for the functions that receive a list of outgoing
/// requests and dispatches those requests.
type KernelSendFn = DMessage list -> Async<unit>

/// The behavior of the kernel. It registers an actor sink and returns
/// to the actor the function to be called to dispatch outgoing requests.
/// It also provides access to the logger of the system.
type IKernel =
    abstract member Register: IActorSink -> Async<KernelSendFn>
    abstract member Logger: ILogger

type DReceiveMessage<'TState> = DMessage -> 'TState -> Async<DMessage list * 'TState>

[<RequireQualifiedAccess>]
module Header =
    let withFrom fid h = { h with Fid = fid }
    let withTo   tid h = { h with Tid = tid }
    let toMe     aid   = Header.Empty |> withFrom (FID aid) |> withTo (TID aid)

    let flip h = let (FID fid), (TID tid) = h.Fid, h.Tid in { Fid = (FID tid); Tid = (TID fid)}

[<RequireQualifiedAccess>]
module DMessage =

    let withHdr  h   m = { m with Hdr = h }
    let withFrom fid m = { m with Hdr = Header.withFrom fid m.Hdr }
    let withTo   tid m = { m with Hdr = Header.withTo tid m.Hdr }
    let withMe   aid m = let hdr = aid |> Header.toMe in withHdr hdr m
    let withPld  pld m = { m with Pld = PLD pld }

    let pld m = let (PLD pld) = m.Pld in pld

    let flip m = { m with Hdr = Header.flip m.Hdr }

[<RequireQualifiedAccess>]
module IActorSink = 

    let postMe (sink: IActorSink) pld = 
        let aid = sink.Aid in
        DMessage.Empty 
        |> DMessage.withMe aid 
        |> DMessage.withPld pld 
        |> sink.Post

[<RequireQualifiedAccess>]
module Log = 
    let err  txt (k: IKernel) = let lgr = k.Logger in lgr.Err  txt
    let info txt (k: IKernel) = let lgr = k.Logger in lgr.Info txt

[<RequireQualifiedAccess>]
module Sys =

    [<RequireQualifiedAccess>]
    module Ids =
        let Logger    = AID "sys:logger"
        let Rebounder = AID "sys:rebounder"