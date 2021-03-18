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

type Packet = {
    Fid: FromId
    Tid: ToId
    Cor: Correlation
    Pld: Payload } with
    override this.ToString() = sprintf "%O->%O %O %O" this.Fid this.Tid this.Cor this.Pld

type IActor =
    abstract member Aid: ActorId

type IKernel =
    abstract member Spawn: ActorId -> Async<IActor>
    abstract member Actors: Async<IActor list>

[<RequireQualifiedAccessAttribute>]
module K =

    type KMessage =
        | KSpawn  of ActorId * AsyncReplyChannel<IActor>
        | KActors of AsyncReplyChannel<IActor list>

    let make () =
        let kmailbox = MailboxProcessor.Start(fun inbox ->
            let rec loop () = async {
                let! msg = inbox.Receive()

                match msg with
                | KSpawn (aid, chnl) ->
                    ()
                | KActors chnl ->
                    ()

                return! loop ()
            }

            loop ()
        )

        let postSpawn aid = kmailbox.PostAndAsyncReply (fun chnl -> KSpawn (aid, chnl))
        let postActors () = kmailbox.PostAndAsyncReply (fun chnl -> KActors chnl)

        { new IKernel with
            member _.Spawn aid = postSpawn aid
            member _.Actors = postActors () }