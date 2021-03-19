namespace Simplee.Distributed

type ActorId = AID of string with
    override this.ToString() = let (AID i) = this in i

type FromId  = FID of ActorId with
    override this.ToString() = let (FID (AID i)) = this in i

type ToId    = TID of ActorId with
    override this.ToString() = let (TID (AID i)) = this in i

type Payload = PLD of byte[] with
    override this.ToString() = let (PLD xs) = this in sprintf "%A" xs

type RequestOut = { 
    Tid: ActorId
    Pld: Payload }

type RequestIn = {
    Fid: FromId
    Tid: ToId
    Pld: Payload }

type IActor =
    abstract member Aid:  Async<ActorId>

type IActorInt =
    abstract member SendRequests: RequestOut list -> Async<unit>
    abstract member CallPublicApi: Payload -> unit

type IActorSink =
    abstract member Aid: ActorId
    abstract member Received: RequestIn -> Async<unit>

type KSend = RequestOut list -> Async<unit>

type IKernel =
    abstract member Register: IActorSink -> Async<KSend>

