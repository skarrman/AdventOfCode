#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <map>

using namespace std;

struct segment
{
  struct point
  {
    int x, y;
    bool operator<(const point &rhs) const
    {
      return tie(x, y) < tie(rhs.x, rhs.y);
    }
  } start, end;
};

std::vector<segment>
read_input()
{
  vector<segment> result;
  ifstream input_file("input.txt");
  string line;

  while (getline(input_file, line))
  {
    size_t i = line.find("-");
    line.replace(i - 1, 4, ",");
    int numbers[4];
    for (int i = 0; i < 4; i++)
    {
      numbers[i] = stoi(line.substr(0, line.find(",")));
      line.erase(0, line.find(",") + 1);
    }
    segment s = {{numbers[0], numbers[1]}, {numbers[2], numbers[3]}};
    result.push_back(s);
  }

  return result;
}
bool is_hor_or_ver(segment s)
{
  return s.start.x == s.end.x || s.start.y == s.end.y;
}

int solve1(vector<segment> input)
{
  vector<segment> filtered;
  copy_if(input.begin(), input.end(), back_inserter(filtered), is_hor_or_ver);
  map<segment::point, int> points;
  for (segment &s : filtered)
  {
    for (int x = min(s.start.x, s.end.x); x <= max(s.start.x, s.end.x); x++)
    {
      for (int y = min(s.start.y, s.end.y); y <= max(s.start.y, s.end.y); y++)
      {
        segment::point p = {x, y};
        points[p] += 1;
      }
    }
  }
  int sum = 0;
  for (auto const &[p, val] : points)
  {
    sum += val > 1 ? 1 : 0;
  }
  return sum;
}

int solve2(vector<segment> input)
{
  map<segment::point, int> points;
  for (segment &s : input)
  {
    int steps = max(abs(s.start.x - s.end.x), abs(s.start.y - s.end.y));
    steps = steps == 0 ? 1 : steps;
    segment::point d = {(s.end.x - s.start.x) / steps, (s.end.y - s.start.y) / steps};
    for (int i = 0; i <= steps; i++)
    {
      segment::point p = {s.start.x + d.x * i, s.start.y + d.y * i};
      points[p] += 1;
    }
  }
  int sum = 0;
  for (auto const &[p, val] : points)
  {
    sum += val > 1 ? 1 : 0;
  }
  return sum;
}

int main()
{
  if (string(getenv("part")) == "part2")
    printf("%d\n", solve2(read_input()));
  else
    printf("%d\n", solve1(read_input()));
  return 0;
}
