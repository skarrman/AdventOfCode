// For more information see https://aka.ms/fsharp-console-apps
open System
open System.Collections.Generic

type point = { x: int; y: int }

type node =
    { point: point
      dir: point
      heatLoss: int
      stepsLeft: int
    // path: node array
    }

let rec dijkstra goal (map: int array array) (queue: PriorityQueue<node, int>) maxSteps =
    let visited = Dictionary<point * point * bool, int>()

    let key n = (n.point, n.dir, n.stepsLeft > 0)

    let moveSteps p s = { x = p.x * s; y = p.y * s }
    let move p d = { x = p.x + d.x; y = p.y + d.y }

    let validPoint p =
        p.y >= 0 && p.y < map.Length && p.x >= 0 && p.x < map[p.y].Length

    let opposite p = { x = -p.x; y = -p.y }

    let allowedStepsInDir n d =
        match n.dir with
        | d' when d' = d -> n.stepsLeft
        | _ -> maxSteps

    let legalDirection n dir =
        match n.dir with
        | d when d = opposite dir -> false
        | _ when allowedStepsInDir n dir <= 0 -> false
        | _ -> true

    let validMoves n =
        [ { x = -1; y = 0 }; { x = 1; y = 0 }; { x = 0; y = -1 }; { x = 0; y = 1 } ]
        |> Seq.filter (fun d -> validPoint (move n.point d))
        |> Seq.filter (legalDirection n)
        |> Seq.map (fun d -> (move n.point d, d))
        |> Seq.map (fun (p, d) ->
            { point = p
              dir = d
              heatLoss = n.heatLoss + map[p.y][p.x]
              stepsLeft = allowedStepsInDir n d - 1
            // path = Array.append n.path [| n |]
            })
        |> Seq.filter (fun n -> (not (visited.ContainsKey(key n))) || visited[key n] > n.heatLoss)
        |> Seq.toList

    let insertSorted nodes =
        // let findIndex n =
        //     match queue.FindIndex(fun n' -> n'.heatLoss > n.heatLoss) with
        //     | -1 -> max 0 (Seq.length queue - 1)
        //     | v -> v

        let rec insert ns =
            match ns with
            | [] -> ()
            | n :: ns' ->
                // queue.Insert((findIndex n), n)
                queue.Enqueue(n, n.heatLoss)
                insert ns'

        insert nodes

    let addMoves next =
        let mutable i = 0

        while i < maxSteps do
            insertSorted (validMoves (moveSteps next i))

            i <- i + 1


    let mutable result = -1

    while result = -1 do
        let next = queue.Dequeue()
        // printfn $"%A{queue.Count}"

        match next with
        | n when n.point = goal -> result <- next.heatLoss
        | n when visited.ContainsKey(key n) && visited[key n] < n.heatLoss -> ()
        | _ ->
            validMoves next |> insertSorted
            visited[key next] <- next.heatLoss

    result




let solutionPart1 (map: int array array) =
    let queue = PriorityQueue<node, int>()

    queue.Enqueue(
        { point = { x = 0; y = 0 }
          dir = { x = 1; y = 1 }
          heatLoss = 0
          stepsLeft = 0 },
        0
    )

    dijkstra
        { x = map[map.Length - 1].Length - 1
          y = map.Length - 1 }
        map
        queue
        3

let solutionPart2 map = 0


[<EntryPoint>]
let main argv =
    let input =
        (System.IO.File.ReadLines("input.txt")
         |> Seq.map (fun row -> row |> Seq.toArray |> Array.map (_.ToString()) |> Array.map int |> Seq.toArray)
         |> Seq.toArray)

    printfn
        "%s"
        (match Environment.GetEnvironmentVariable("part") with
         | null
         | "part1" -> input |> solutionPart1 |> string
         | "part2" -> input |> solutionPart2 |> string
         | env -> $"Unknown value {env}")

    0 // return an integer exit code


// | n when Map.exists (fun k (h, m) -> k = (n.point, n.dir) && h < n.heatLoss && m >= validMoves.Length) visited ->
//     dijkstra goal map queue[1..] visited
// | _ ->
//     dijkstra
//         goal
//         map
//         (validMoves |> Array.append queue[1..] |> Array.sortBy (_.heatLoss))
//         (Map.add (next.point, next.dir) (next.heatLoss, validMoves.Length) visited)

// | n when Map.exists (fun k h -> k = (n.point, n.dir) && h <= n.heatLoss) visited ->
//     dijkstra goal map queue[1..] visited
// | _ ->
//     dijkstra
//         goal
//         map
//         (validMoves next |> List.append queue[1..] |> List.sortBy (_.heatLoss))
//         (Map.add (next.point, next.dir) next.heatLoss visited)
