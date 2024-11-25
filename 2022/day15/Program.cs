using System.Text.RegularExpressions;

var pattern = @"x=(-?\d+),\sy=(-?\d+)[^x]+x=(-?\d+),\sy=(-?\d+)";
var pairs = File.ReadLines("input.txt").Select(line =>
{
    var groups = Regex.Matches(line, pattern).First().Groups.Values.ToArray();
    return ((long.Parse(groups[1].Value), long.Parse(groups[2].Value)),
        (long.Parse(groups[3].Value), long.Parse(groups[4].Value)));
});

long Dist((long x, long y) fst, (long x, long y) snd) => Math.Abs(fst.x - snd.x) + Math.Abs(fst.y - snd.y);
bool InRange((long x, long y) p, long max) => 0 <= p.x && p.x <= max && 0 <= p.y && p.y <= max;


long Solution1(((long x, long y), (long x, long y))[] sensorsAndBeacons, long relevantY)
{
    var beacons = sensorsAndBeacons.Select(tuple => tuple.Item2).ToHashSet();
    var noBeacons = new HashSet<(long x, long y)>();
    foreach (var (sensor, beacon) in sensorsAndBeacons)
    {
        var dist = Dist(sensor, beacon);
        for (var x = sensor.x - dist; x < sensor.x + dist; x++)
            if (Dist((x, relevantY), sensor) <= dist && !beacons.Contains((x, relevantY)))
                noBeacons.Add((x, relevantY));
    }

    return noBeacons.Count(p => p.y == relevantY);
}

long Solution2(((long x, long y), (long x, long y))[] sensorsAndBeacons, long max)
{
    foreach (var (sensor, beacon) in sensorsAndBeacons)
    {
        var dist = (int) Dist(sensor, beacon) + 1;
        for (var d = -dist; d <= dist; d++)
        {
            var maybe = new[]
                {
                    (x: sensor.x + d, y: sensor.y + (dist - Math.Abs(d))),
                    (x: sensor.x + d, y: sensor.y - (dist - Math.Abs(d)))
                }.Select(p => (p, p.x * 4000000 + p.y))
                .Where(tuple =>
                    InRange(tuple.p, max) &&
                    sensorsAndBeacons.All(sb => Dist(tuple.p, sb.Item1) > Dist(sb.Item1, sb.Item2)))
                .Select(t => t.Item2)
                .FirstOrDefault();
            if (maybe != default)
                return maybe;
        }
    }

    throw new Exception();
}

var solution = Environment.GetEnvironmentVariable("part") switch
{
    "part2" => Solution2(pairs.ToArray(), 4000000),
    _ => Solution1(pairs.ToArray(), 2000000)
};

Console.WriteLine(solution);