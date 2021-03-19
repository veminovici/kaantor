namespace Simplee

[<RequireQualifiedAccessAttribute>]
module Async =
    let private chain (x: Async<unit>) (y: Async<unit>) = async {
        do! x
        do! y }

    let ureduce (xs: Async<unit> list) =
        xs
        |> List.reduce chain

    let map f a = async {
        let! r = a
        return f r
    }
