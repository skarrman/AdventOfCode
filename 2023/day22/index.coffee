fs = require 'fs'

input_data_lines = () -> 
    rows = fs.readFileSync("input.txt", "utf8").toString().split("\n").map (r) -> r.split("~").flatMap (p) -> p.split(",").map (n) -> parseInt(n)
    rows.sort((lhs,rhs) -> lhs[2]-rhs[2])
    rows.map (r, i) -> {id: i, p1: {x: r[0], y: r[1],z: r[2]}, p2: {x: r[3], y: r[4], z: r[5]}}

find_supporting = (blocks, block) ->
    below = blocks.filter (b) -> (block.p1.z - 1) == b.p2.z
    i = 0
    supporting = 0
    while i < below.length
        if below[i].p1.x <= block.p2.x && below[i].p2.x >= block.p1.x && below[i].p1.y <= block.p2.y && below[i].p2.y >= block.p1.y
            supporting++
        i++
    supporting

find_supported = (blocks, block) ->
    blocks.filter (b) -> (block.p2.z + 1) == b.p1.z && b.p1.x <= block.p2.x && b.p2.x >= block.p1.x && b.p1.y <= block.p2.y && b.p2.y >= block.p1.y

apply_gravity = (blocks) -> 
    moved = 0
    i = 0
    while i < blocks.length
        if blocks[i].p1.z != 1 && blocks[i].p2.z != 1 && find_supporting(blocks, blocks[i]) == 0
            blocks[i].p1.z--    
            blocks[i].p2.z--    
            moved++
        i++
    moved

get_solution = (snd) -> 
    blocks = input_data_lines()
    while true
        if apply_gravity(blocks) == 0
            break
    if snd
        blocks.sort (a,b) -> a.p1.z - b.p1.z
        fall = 0
        i = 0
        while i < blocks.length
            fall += apply_gravity(JSON.parse(JSON.stringify(blocks.filter (b) -> b.id != i)))
            i++
        fall

    else
        disintegratable = 0
        i = 0
        while i < blocks.length
            supporing_of_supported = find_supported(blocks, blocks[i]).flatMap (b) -> find_supporting(blocks, b)
            if supporing_of_supported.length == 0 || supporing_of_supported.every (s) -> s > 1
                disintegratable++
            i++
        disintegratable

console.log get_solution(process.env.part == "part2")
