module Aoc

let filterDir (input: (string * int) []) (dir: string) = input |> Array.filter (fun (str, _) -> str.Equals dir)|> Array.sumBy snd

let solutionPart1 (input: (string * int) []) = (filterDir input "forward") * ((filterDir input "down") - (filterDir input "up"))

let solutionPart2 (input: List<string * int>) =
    let rec move (lst: List<string * int>) (aim: int) (hor: int) (depth: int) =
        match lst with
        | curr :: next ->
            match curr with
            | "forward", n -> move next aim (hor + n) (depth + (n * aim))
            | "up", n -> move next (aim - n) hor depth
            | "down", n -> move next (aim + n) hor depth
            | _ -> -1
        | [] -> hor * depth

    move input 0 0 0
