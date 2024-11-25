var pairs = File.ReadLines("input.txt").Select(line =>
{
    var elves = line.Split(",");
    var e1 = elves[0].Split("-");
    var e2 = elves[1].Split("-");
    return ((int.Parse(e1[0]), int.Parse(e1[1])), (int.Parse(e2[0]), int.Parse(e2[1])));
});

var solution = Environment.GetEnvironmentVariable("part") switch
{
    "part2" => pairs.Aggregate(0,
        (i, tuple) => IsOverlapping(tuple.Item1, tuple.Item2) || IsOverlapping(tuple.Item2, tuple.Item1)
            ? i + 1 : i),
    _ => pairs.Aggregate(0,
        (i, tuple) => IsTotallyContain(tuple.Item1, tuple.Item2) || IsTotallyContain(tuple.Item2, tuple.Item1)
            ? i + 1 : i)
};

bool IsTotallyContain((int, int) e1, (int, int) e2) => e1.Item1 <= e2.Item1 && e1.Item2 >= e2.Item2;

bool IsOverlapping((int, int) e1, (int, int) e2) => e1.Item1 <= e2.Item1 && e1.Item2 >= e2.Item1 || e1.Item1 <= e2.Item2 && e1.Item2 >= e2.Item2;

Console.WriteLine(solution);