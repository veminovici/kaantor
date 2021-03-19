namespace Simplee.Distributed

type ActorId = AID of string with
    override this.ToString() = let (AID i) = this in i

type FromId  = FID of ActorId with
    override this.ToString() = let (FID (AID i)) = this in i

type ToId    = TID of ActorId with
    override this.ToString() = let (TID (AID i)) = this in i

type Header = {
    Fid: FromId
    Tid: ToId }

type Payload = PLD of byte[] with
    override this.ToString() = let (PLD xs) = this in sprintf "%A" xs

type RequestOut = { 
    Tid: ActorId
    Pld: Payload }

type RequestIn = {
    Hdr: Header
    Pld: Payload }

type IActor =
    abstract member Aid:  ActorId

type IActorInt =
    abstract member SendRequests: RequestOut list -> Async<unit>
    abstract member Api: obj -> Async<obj>

type IActorSink =
    abstract member Aid: ActorId
    abstract member Received: RequestIn -> Async<unit>

type ActorApiHndl<'a> = obj -> 'a -> obj * 'a
type ActorMsgHndl<'a> = RequestIn -> 'a -> RequestOut list * 'a

type LEntry =
    | LErr  of string
    | LInfo of string

type ILogger =
    inherit IActor
    abstract member Info: string -> unit
    abstract member Err:  string -> unit
    abstract member Logs: Async<LEntry list>

type KernelSendFn = RequestOut list -> Async<unit>

type IKernel =
    abstract member Register: IActorSink -> Async<KernelSendFn>
    abstract member Logger: ILogger

