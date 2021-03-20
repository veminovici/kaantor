namespace Simplee.Distributed

/// The actor identifier.
type ActorId = AID of string with
    override this.ToString() = let (AID i) = this in i

/// The identifier of the source of the message
type FromId  = FID of ActorId with
    override this.ToString() = let (FID (AID i)) = this in i

/// The identifier of the destination of the message
type ToId    = TID of ActorId with
    override this.ToString() = let (TID (AID i)) = this in i

/// The header of the message
type Header = {
    Fid: FromId
    Tid: ToId }

/// The payload of the message. The payload is just an array of bytes.
type Payload = PLD of byte[] with
    override this.ToString() = let (PLD xs) = this in sprintf "%A" xs

/// A request containing a payload to be sent to a given actor.
type RequestOut = { 
    Tid: ActorId
    Pld: Payload }

/// A received request, with a given payload, from an actor.
type RequestIn = {
    Hdr: Header
    Pld: Payload }

/// The behavior of an actor. It just needs to return its identity.
type IActor =
    abstract member Aid:  ActorId

/// The internal behavior of an actor. 
/// An actor should be able to send requests.
/// Also an actor should able to process public api calls, which could
/// have arguments and optionally return a result.
type IActorInt =
    abstract member SendRequests: RequestOut list -> Async<unit>
    abstract member Api: obj -> Async<obj>

/// The actor sink behavior.
/// The actor should be able to identify itself.
/// The actor should be able to receive messages.
type IActorSink =
    abstract member Aid: ActorId
    abstract member Received: RequestIn -> Async<unit>

/// A handle function that defines the actor behavior
/// when a public api call is placed. The handler receives
/// the the arguments and the actor's internal state and returns
/// the optional result and the new state.
type ActorApiHndl<'TState> = obj -> 'TState -> obj * (RequestOut list) * 'TState

/// A handle function that defines the actor behavior
/// when a message is received. The handler receives
// the incoming requests and the actor's internal state and returns
// the list of requests to be sent out and the new internal state.
type ActorMsgHndl<'a> = RequestIn -> 'a -> RequestOut list * 'a

/// The log entry.
type LEntry =
    | LErr  of string
    | LInfo of string

/// The behavior of the logger actor. It inherits the actor
/// general behavior. It also can log an info, or an err, and
/// it can return the recorded entries.
type ILogger =
    inherit IActor
    abstract member Info: string -> unit
    abstract member Err:  string -> unit
    abstract member Logs: Async<LEntry list>

/// An alias for the functions that receive a list of outgoing
/// requests and dispatches those requests.
type KernelSendFn = RequestOut list -> Async<unit>

/// The behavior of the kernel. It registers an actor sink and returns
/// to the actor the function to be called to dispatch outgoing requests.
/// It also provides access to the logger of the system.
type IKernel =
    abstract member Register: IActorSink -> Async<KernelSendFn>
    abstract member Logger: ILogger