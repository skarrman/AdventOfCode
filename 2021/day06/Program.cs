// See https://aka.ms/new-console-template for more information

Console.WriteLine(
    Enumerable.Range(0, Environment.GetEnvironmentVariable("part") == "part1" ? 80 : 256)
        .Aggregate(File.ReadAllText("input.txt").Split(",").Select(Int64.Parse).Aggregate(
            Enumerable.Range(0, 9).ToDictionary(x => (long) x, _ => 0L), (d, f) =>
            {
                d[f]++;
                return d;
            }), (current, _) => Enumerable.Range(0, 9)
            .ToDictionary(x => (long) x, x => x switch
            {
                6 => current[7] + current[0],
                8 => current[0],
                var n => current[n + 1]
            })).Values.Sum());