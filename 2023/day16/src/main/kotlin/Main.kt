package sk.aoc

import java.io.File

fun main() {
    val input = File("input.txt").readLines().map { it.toCharArray().toList() }
    val answer = when (System.getenv("part")) {
        "part2" -> solutionPart2(input)
        else -> solutionPart1(input)
    }
    println(answer)
}

enum class Direction(val d: Pair<Int, Int>) {
    Right(Pair(1, 0)),
    Down(Pair(0, 1)),
    Left(Pair(-1, 0)),
    Up(Pair(0, -1))
};

fun energize(map: List<List<Char>>, initial: Pair<Pair<Int, Int>, Direction>): Int {
    val energized: MutableSet<Pair<Pair<Int, Int>, Direction>> = hashSetOf();
    var beams = List(1, init = { initial })
    while (beams.isNotEmpty()) {
        val nextBeams: MutableList<Pair<Pair<Int, Int>, Direction>> = arrayListOf()
        for (beam in beams) {
            val next = Pair(beam.first.first + beam.second.d.first, beam.first.second + beam.second.d.second)
            if (energized.contains(beam))
                continue
            energized.add(beam)
            if (next.second < 0 || next.second >= map.size || next.first < 0 || next.first >= map[next.second].size)
                continue
            when (map[next.second][next.first]) {
                '.' -> nextBeams.add(Pair(next, beam.second))
                '/' -> when (beam.second) {
                    Direction.Right -> nextBeams.add(Pair(next, Direction.Up))
                    Direction.Down -> nextBeams.add(Pair(next, Direction.Left))
                    Direction.Left -> nextBeams.add(Pair(next, Direction.Down))
                    Direction.Up -> nextBeams.add(Pair(next, Direction.Right))
                }

                '\\' -> when (beam.second) {
                    Direction.Right -> nextBeams.add(Pair(next, Direction.Down))
                    Direction.Down -> nextBeams.add(Pair(next, Direction.Right))
                    Direction.Left -> nextBeams.add(Pair(next, Direction.Up))
                    Direction.Up -> nextBeams.add(Pair(next, Direction.Left))
                }

                '|' -> when (beam.second) {
                    Direction.Right, Direction.Left -> {
                        nextBeams.add(Pair(next, Direction.Up))
                        nextBeams.add(Pair(next, Direction.Down))
                    }

                    Direction.Down, Direction.Up -> nextBeams.add(Pair(next, beam.second))
                }

                '-' -> when (beam.second) {
                    Direction.Right, Direction.Left -> nextBeams.add(Pair(next, beam.second))
                    Direction.Down, Direction.Up -> {
                        nextBeams.add(Pair(next, Direction.Right))
                        nextBeams.add(Pair(next, Direction.Left))
                    }
                }
            }
        }
        beams = nextBeams
    }
    return energized.map { it.first }.toSet().size - 1
}

fun solutionPart1(map: List<List<Char>>): Int = energize(map, Pair(Pair(-1, 0), Direction.Right))

fun solutionPart2(map: List<List<Char>>): Int {
    val top = map.indices.map { Pair(Pair(it, -1), Direction.Down) }
    val bottom = map.indices.map { Pair(Pair(it, map.size), Direction.Up) }
    val left = map[0].indices.map { Pair(Pair(-1, it), Direction.Right) }
    val right = map[0].indices.map { Pair(Pair(map[0].size, it), Direction.Left) }
    return (top + bottom + left + right).maxOfOrNull { energize(map, it) }!!
}