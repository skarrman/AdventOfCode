open System
open Aoc
[<EntryPoint>]
let main argv =
    let input = (System.IO.File.ReadLines("input.txt") |> Seq.map (fun str -> str.Split " ") |> Seq.map (fun sts -> (sts[0], int (sts[1])))|> Seq.toArray)
    
    printfn "F#\n%s" (
        match Environment.GetEnvironmentVariable("part") with
        | null | "part1" -> input |> solutionPart1 |> string
        | "part2" -> input |> Array.toList |> solutionPart2 |> string
        | env -> $"Unknown value {env}"
    )
    0 // return an integer exit code