
type ApiCall   = ApiCall of obj

type KernelMsg =
    | Pid of AsyncReplyChannel<string>
    | Api of ApiCall

type IProcessPub =
    abstract Pid: Async<string>
    abstract Api: (ApiCall -> unit)

module Kernel =

    let create pid = 
    
        let mbox = MailboxProcessor.Start (fun inbox -> 
    
            let rec loop () = async {
                match! inbox.Receive () with
                | Pid rply    -> 
                    rply.Reply(pid)
                | Api (ApiCall (call)) ->
                    // call should be send to the other people 
                    ()

                return! loop () }

            loop ())

        let pid = mbox.PostAndAsyncReply(Pid)
        let api = Api >> mbox.Post

        { new IProcessPub with
            member _.Pid = pid
            member _.Api = api }

open Kernel

"myPid"
|> create
|> fun proc -> proc.Pid
|> Async.RunSynchronously
|> printfn "Pid=%s"

"myPid"
|> create
|> fun proc -> proc.Api
|> fun f -> f (ApiCall 1)
