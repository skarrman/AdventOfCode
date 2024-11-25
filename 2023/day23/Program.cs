using System.Collections;

var input = File.ReadLines("input.txt").Select(l => l.Select(c => c).ToArray()).ToArray();

Console.WriteLine(Environment.GetEnvironmentVariable("part") switch
{
    "part2" => LongestPathWithSlopesAsPath(input),
    _ => LongestPathWithSlopes(input),
});
return;

void TopologicalSort((int, int) node, Dictionary<(int, int), List<(int, int)>> nodes, HashSet<(int, int)> visited,
    List<(int, int)> stack)
{
    var localStack = new List<((int, int), List<(int, int)>)> {(node, nodes[node])};

    while (localStack.Any())
    {
        var (n, nAdj) = localStack.Last();
        localStack.RemoveAt(localStack.Count - 1);
        visited.Add(n);
        var done = true;
        foreach (var adj in nAdj.Where(adj => !visited.Contains(adj)))
        {
            done = false;
            localStack.Add((n, nAdj));
            localStack.Add((adj, nodes[adj]));
            break;
        }

        if (done)
        {
            stack.Add(n);
        }
    }
}

int LongestPathWithSlopes(char[][] hikeMap)
{
    var nodes = new Dictionary<(int, int), List<(int, int)>>();
    foreach (var (l, y) in hikeMap.Zip(Enumerable.Range(0, hikeMap.Length)))
    {
        foreach (var (c, x) in l.Zip(Enumerable.Range(0, hikeMap[y].Length)))
        {
            if (c == '#')
                continue;
            var adj = new List<(int, int)>();
            if (c == '.')
            {
                (int, int)[] dirs = {(0, 1), (1, 0), (-1, 0), (0, -1)};
                foreach (var (dx, dy) in dirs)
                {
                    if (y + dy >= 0 && y + dy < hikeMap.Length && x + dx >= 0 && x + dx <
                        hikeMap[y].Length && hikeMap[y + dy][x + dx] != '#')
                    {
                        if (hikeMap[y + dy][x + dx] == '.' || (hikeMap[y + dy][x + dx] == '<' && dx != 1) || (
                                hikeMap[y + dy][x + dx] == '>' && dx != -1) ||
                            (hikeMap[y + dy][x + dx] == '^' && dy != 1) || (
                                hikeMap[y + dy][x + dx] == 'v' && dy != -1))
                            adj.Add((x + dx, y + dy));
                    }
                }
            }
            else if (c == '<')
                adj.Add((x - 1, y));
            else if (c == '>')
                adj.Add((x + 1, y));
            else if (c == '^')
                adj.Add((x, y - 1));
            else if (c == 'v')
                adj.Add((x, y + 1));
            else
                Console.WriteLine($"Unhandled {c}");

            nodes[(x, y)] = adj;
        }
    }

    var start = (0, 0);
    var end = (0, 0);
    foreach (var (x, y) in nodes.Keys)
    {
        if (y == 0)
            start = (x, y);
        if (y == hikeMap.Length - 1)
            end = (x, y);
    }

    var visited = new HashSet<(int, int)>();
    var stack = new List<(int, int)>();

    foreach (var node in nodes.Keys.Where(node => !visited.Contains(node)))
        TopologicalSort(node, nodes, visited, stack);

    var distances = new Dictionary<(int, int), int>() {{start, 0}};

    while (stack.Count > 0)
    {
        var node = stack.Last();
        stack.RemoveAt(stack.Count - 1);

        foreach (var adj in nodes[node])
            if (!distances.TryGetValue(adj, out var value) || value < distances[node] + 1)
                distances[adj] = distances[node] + 1;
    }

    return distances[end];
}

int LongestPathWithSlopesAsPath(char[][] hikeMap)
{
    var nodes = new Dictionary<(int, int), List<(int, int)>>();
    foreach (var (l, y) in hikeMap.Zip(Enumerable.Range(0, hikeMap.Length)))
    {
        foreach (var (c, x) in l.Zip(Enumerable.Range(0, hikeMap[y].Length)))
        {
            if (c == '#')
                continue;
            var adj = new List<(int, int)>();

            (int, int)[] dirs = {(0, 1), (1, 0), (-1, 0), (0, -1)};
            foreach (var (dx, dy) in dirs)
            {
                if (y + dy >= 0 && y + dy < hikeMap.Length && x + dx >= 0 && x + dx <
                    hikeMap[y].Length && hikeMap[y + dy][x + dx] != '#')
                {
                    adj.Add((x + dx, y + dy));
                }
            }

            nodes[(x, y)] = adj;
        }
    }

    var start = (0, 0);
    var end = (0, 0);
    foreach (var (x, y) in nodes.Keys)
    {
        if (y == 0)
            start = (x, y);
        if (y == hikeMap.Length - 1)
            end = (x, y);
    }

    var contractedNodes = new Dictionary<(int, int), List<((int, int), int)>>();

    foreach (var (node, adjs) in nodes.Where(n => n.Key == start || n.Value.Count > 2))
    {
        contractedNodes[node] = [];
        foreach (var adj in adjs)
        {
            var length = 0;
            var curr = node;
            var next = adj;
            while (nodes[next].Count == 2)
            {
                length++;
                var curr1 = curr;
                var tmp = nodes[next].First(a => a != curr1);
                curr = next;
                next = tmp;
            }

            contractedNodes[node].Add((next, length + 1));
        }
    }

    var stack = new Stack<((int, int), int, HashSet<(int, int)>, HashSet<(int, int)>)>();
    stack.Push((start, 0, [], []));

    var longest = 0;
    while (stack.Count > 0)
    {
        var (n, length, visited, adjSearched) = stack.Pop();

        if (n == end)
        {
            if (length > longest)
            {
                longest = length;
            }

            continue;
        }

        visited.Add(n);

        foreach (var (adj, len) in contractedNodes[n]
                     .Where(adj => !visited.Contains(adj.Item1) && !adjSearched.Contains(adj.Item1)))
        {
            var adjVisited = new HashSet<(int, int)>(visited);
            adjSearched.Add(adj);
            stack.Push((n, length, visited, adjSearched));
            stack.Push((adj, length + len, adjVisited, []));
            break;
        }
    }

    return longest;
}