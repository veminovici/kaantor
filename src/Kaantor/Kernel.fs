namespace Simplee.Distributed

type ActorId = AID of string with
    override this.ToString() = let (AID i) = this in i

type FromId  = FID of ActorId with
    override this.ToString() = let (FID (AID i)) = this in i

type ToId    = TID of ActorId with
    override this.ToString() = let (TID (AID i)) = this in i

type Payload = PLD of byte[] with
    override this.ToString() = let (PLD xs) = this in sprintf "%A" xs

type Correlation = 
    | Cor   of string
    | CorA  of string * string
    | CorI  of string * int
    | CorAI of string * string * int
    with
    override this.ToString() =
        match this with
        | Cor c -> c
        | CorA (c, a) -> sprintf "(%s:%s)" c a
        | CorI (c, i) -> sprintf "(%s:%d)" c i
        | CorAI (c, a, i) -> sprintf "(%s:%s:%d)" c a i

type RequestIn = {
    Fid: FromId
    Cor: Correlation
    Pld: Payload } with
    override this.ToString() = sprintf "%O-> %O %O" this.Fid this.Cor this.Pld

type RequestOut = {
    Tid: ToId
    Cor: Correlation
    Pld: Payload } with
    override this.ToString() = sprintf "->%O %O %O" this.Tid this.Cor this.Pld

type IActor =
    abstract member Aid: ActorId
    abstract member Post: RequestOut -> Async<unit>

type IKernel =
    abstract member Spawn: ActorId -> Async<IActor>
    abstract member Actors: Async<IActor list>

[<RequireQualifiedAccessAttribute>]
module K =

    type private IActorSink = 
        abstract member Post: RequestIn -> Async<unit>

    type private Packet = {
        Fid: FromId
        Tid: ToId
        Cor: Correlation
        Pld: Payload } with
        override this.ToString() = sprintf "%O->%O %O %O" this.Fid this.Tid this.Cor this.Pld

    type private IKernelSink =
        abstract member Post: Packet -> Async<unit>

    type private KMessage =
        | KSpawn  of ActorId * AsyncReplyChannel<IActor>
        | KActors of AsyncReplyChannel<IActor list>
        | KPacket of Packet

    let private pktOfReq aid (req: RequestOut) = { 
        Fid = FID aid
        Tid = req.Tid
        Cor = req.Cor
        Pld = req.Pld }

    let private pktToReq (pkt: Packet) : RequestIn = {
        Fid = pkt.Fid
        Cor = pkt.Cor
        Pld = pkt.Pld }

    let make () =
        let mutable ksink = Unchecked.defaultof<IKernelSink>

        let kmailbox = MailboxProcessor.Start(fun inbox ->

            let mkIActor aid = { new IActor with
                member _.Aid = aid
                member _.Post req = req |> pktOfReq aid |> ksink.Post }

            let mkIActorSink = { new IActorSink with
                member _.Post req = async {return () }}

            let mkActor aid = 
                let a = mkIActor aid
                let snk = mkIActorSink
                a, snk

            let rec loop actors = async {

                match! inbox.Receive() with
                | KSpawn (aid, chnl) ->
                    let (a, snk) = mkActor aid
                    a |> chnl.Reply
                    return! loop ((a, snk) :: actors)
                | KActors chnl ->
                    actors |> List.map fst |> chnl.Reply
                    return! loop actors
                | KPacket pkt ->
                    let (_, snk: IActorSink) = actors |> List.where (fun (a: IActor, _) -> (TID a.Aid) = pkt.Tid) |> List.head
                    pkt |> pktToReq |> snk.Post |> Async.RunSynchronously
                    return! loop actors
            }

            loop []
        )

        let postSpawn  aid = kmailbox.PostAndAsyncReply (fun chnl -> KSpawn (aid, chnl))
        let postActors  () = kmailbox.PostAndAsyncReply (fun chnl -> KActors chnl)
        let postPacket pkt = async { return kmailbox.Post (KPacket pkt) }

        ksink <- { new IKernelSink with
            member _.Post pkt = postPacket pkt }

        { new IKernel with
            member _.Spawn aid = postSpawn aid
            member _.Actors = postActors () }

    let spawn aid (k: IKernel) = k.Spawn aid
    let actors (k: IKernel) = k.Actors