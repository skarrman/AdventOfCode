import java.io.File
import kotlin.math.abs
import kotlin.math.max


fun main() {
    val input = File("input.txt").readLines().map {
        val p = it.split(" ")
        Pair(p[0], p[1].toInt())
    }
    val answer = when (System.getenv("part")) {
        "part2" -> solution(input, 9)
        else -> solution(input, 1)
    }
    println(answer)
}

fun solution(input: List<Pair<String, Int>>, length: Int): Int {
    val tailPositions = HashSet<Pair<Int, Int>>()
    var headPos = Pair(0, 0)
    val tails = Array(length) { Pair(0, 0) }
    for (move in input) {
        for (i in 0 until move.second) {
            when (move.first) {
                "R" -> headPos = headPos.copy(first = headPos.first + 1)
                "L" -> headPos = headPos.copy(first = headPos.first - 1)
                "U" -> headPos = headPos.copy(second = headPos.second - 1)
                "D" -> headPos = headPos.copy(second = headPos.second + 1)
            }
            var front = headPos
            for (i in tails.indices) {
                val body = tails[i].copy()
                if (max(abs(front.first - body.first), abs(front.second - body.second)) > 1) {
                    var d = Pair(0, 0)
                    if (front.first < body.first)
                        d = d.copy(first = -1)
                    if (front.first > body.first)
                        d = d.copy(first = 1)
                    if (front.second < body.second)
                        d = d.copy(second = -1)
                    if (front.second > body.second)
                        d = d.copy(second = 1)
                    tails[i] = body.copy(first = body.first + d.first, second = body.second + d.second)
                }
                front = tails[i]
            }
            tailPositions.add(tails.last())
        }
    }
    return tailPositions.size
}