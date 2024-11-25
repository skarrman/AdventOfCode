import scala.io.Source

def calc(time: Long, dist: Long) : Long =
    var wins : Long = 0
    for(t <- 1L to time){
        wins = wins + (if (t * (time - t) > dist) 1 else 0)
    }
    wins

def part1(times: Array[Long], distances: Array[Long]) : Long = 
    var tot : Long = 1
    for(i <- 0 to times.length-1){
        val time = times(i)
        val dist = distances(i)
        
        val wins = calc(time, dist)        
        if wins > 0 then tot *= wins
    }
    tot

def part2(times: Array[Long], distances: Array[Long]) : Long =
    var time = ""
    var dist = ""

    for(i <- 0 to times.length-1){
        time = s"$time${times(i)}"
        dist = s"$dist${distances(i)}"
    }
    calc(time.toLong, dist.toLong)

@main def main() =
    val fileContent = Source.fromFile("input.txt").getLines.toArray
    val times = fileContent(0).split(":")(1).split(" ").filter(_.nonEmpty).map(_.toLong)
    val distances = fileContent(1).split(":")(1).split(" ").filter(_.nonEmpty).map(_.toLong)
    
    val part = sys.env.get("part").getOrElse("part1")
    val result = if part == "part2" then part2(times, distances) else part1(times, distances)

    println(result)