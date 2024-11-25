open System
open System.Linq

let games =
    (System.IO.File.ReadLines("input.txt")
     |> List.ofSeq)

let sol1 hands =
    match hands with
    | "A X" -> 3 + 1
    | "A Y" -> 6 + 2
    | "A Z" -> 0 + 3
    | "B X" -> 0 + 1
    | "B Y" -> 3 + 2
    | "B Z" -> 6 + 3
    | "C X" -> 6 + 1
    | "C Y" -> 0 + 2
    | "C Z" -> 3 + 3
    | _ -> failwith "Unrecognized game"

let sol2 hands =
    match hands with
    | "A X" -> 0 + 3
    | "A Y" -> 3 + 1
    | "A Z" -> 6 + 2
    | "B X" -> 0 + 1
    | "B Y" -> 3 + 2
    | "B Z" -> 6 + 3
    | "C X" -> 0 + 2
    | "C Y" -> 3 + 3
    | "C Z" -> 6 + 1
    | _ -> failwith "Unrecognized game"

let rec result games scoringFn score  =
    match games with
    | [] -> score
    | head :: tail -> result tail scoringFn (score + scoringFn head) 

printfn
    "%d"
    (match Environment.GetEnvironmentVariable("part") with
     | "part2" -> (result games sol2 0)
     | _ -> (result games sol1 0))
