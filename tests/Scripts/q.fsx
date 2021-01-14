#r "nuget: FsPickler"
#r "nuget: FsPickler.Json"
#r "nuget: FSharp.Quotations.Evaluator"

open MBrace.FsPickler
open MBrace.FsPickler.Json

open FSharp.Quotations
open FSharp.Quotations.Evaluator

open System.IO
open System.Text

let serializer = JsonSerializer(indent = true)
let utf8 = UTF8Encoding(false)

let toJson x =
    use stream = new MemoryStream()
    serializer.Serialize(stream, x)
    stream.ToArray() |> utf8.GetString

let parseJson<'a> json =
    use reader = new StringReader(json)
    //serializer.Deserialize<'a>(reader)
    serializer.Deserialize(reader)

let fn x y = x + y

let f = <@ fn 10 @>
let serialized = toJson f

printfn "Serialized:\n%s" serialized

//let deserialized = parseJson<Expr<int -> int>> serialized
let deserialized: Expr<int -> int> = parseJson serialized

let increment = deserialized |> QuotationEvaluator.Evaluate

1
|> increment
|> printfn "Res: %d"
 