namespace Simplee

/// Expands the Async monad behavior.

[<RequireQualifiedAccessAttribute>]
module Async =

    let private rtrn = async.Return

    let private reducer f (x: Async<'a>) (y: Async<'a>) : Async<'a> = async {
        let! x = x
        let! y = y
        return f x y }

    let reduce f (xs: Async<'a> list) = xs |> List.reduce f

    let private reducerU = reducer (fun _ _ -> ())

    let reduceU = reduce reducerU

    /// Maps the result of an async expression using a given function.
    let map f a = async {
        let! r = a
        return f r }

    /// Applies a function to an async flow.
    let apply f a = async {
        let! f = f
        let! a = a
        return f a }

    /// Maps the results of two async flows.
    let map2 f x y = apply (apply (rtrn f) x) y

    /// Zips the results of two async flows.
    let zip x y = map2 (fun x y -> x, y) x y