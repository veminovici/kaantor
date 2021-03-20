namespace Simplee

/// Expands the Async monad behavior.

[<RequireQualifiedAccessAttribute>]
module Async =

    /// Chains two async expressions which return unit. The composes
    /// async expression will also return the unit.
    let private chain (x: Async<unit>) (y: Async<unit>) = async {
        do! x
        do! y }

    /// Reduces a list of async expressions which return unit to one
    /// async expression which also returns unit.
    let ureduce (xs: Async<unit> list) =
        xs
        |> List.reduce chain

    /// Maps the result of an async expression using a given function.
    let map f a = async {
        let! r = a
        return f r }
