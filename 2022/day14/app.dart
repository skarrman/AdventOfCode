import 'dart:collection';
import 'dart:ffi';
import 'dart:io';
import 'dart:convert';
import 'dart:async';

import 'dart:math';

int getMaxY(Map<Point, String> map) {
  return map.entries.fold(
      0,
      (value, element) => element.value == "#" && element.key.y > value
          ? element.key.y
          : value);
}

int solutionPart1(Map<Point, String> map) {
  int maxY = getMaxY(map);
  var noFreeFall = true;
  while (noFreeFall) {
    var pos = Point(500, 0);
    while (true) {
      if (!map.containsKey(Point(pos.x, pos.y + 1)))
        pos = Point(pos.x, pos.y + 1);
      else if (!map.containsKey(Point(pos.x - 1, pos.y + 1)))
        pos = Point(pos.x - 1, pos.y + 1);
      else if (!map.containsKey(Point(pos.x + 1, pos.y + 1)))
        pos = Point(pos.x + 1, pos.y + 1);
      else {
        map[pos] = "o";
        break;
      }
      if (pos.y >= maxY) {
        noFreeFall = false;
        break;
      }
    }
  }
  return map.entries.where((element) => element.value == "o").length;
}

int solutionPart2(Map<Point, String> map) {
  int maxY = getMaxY(map) + 2;
  while (!map.containsKey(Point(500, 0))) {
    var pos = Point(500, 0);
    while (true) {
      if (pos.y + 1 == maxY) {
        map[pos] = "o";
        break;
      }
      if (!map.containsKey(Point(pos.x, pos.y + 1)))
        pos = Point(pos.x, pos.y + 1);
      else if (!map.containsKey(Point(pos.x - 1, pos.y + 1)))
        pos = Point(pos.x - 1, pos.y + 1);
      else if (!map.containsKey(Point(pos.x + 1, pos.y + 1)))
        pos = Point(pos.x + 1, pos.y + 1);
      else {
        map[pos] = "o";
        break;
      }
    }
  }
  return map.entries.where((element) => element.value == "o").length;
}

void main() async {
  Map<Point, String> input = await parseInput("input.txt");
  String part = Platform.environment["part"] ?? "part1";
  if (part == "part1") {
    print(solutionPart1(input));
  } else if (part == "part2") {
    print(solutionPart2(input));
  } else {
    print("Unknown part " + part);
  }
}

Future<Map<Point, String>> parseInput(String path) async {
  var lines = await new File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(const LineSplitter())
      .toList();
  var map = HashMap<Point, String>();
  for (var line in lines) {
    var points = line.split(" -> ").map((e) {
      var p = e.split(",");
      return Point(int.parse(p[0]), int.parse(p[1]));
    }).toList();
    for (var i = 1; i < points.length; i++) {
      var from = points[i - 1];
      var to = points[i];
      for (var y = min(from.y, to.y); y <= max(from.y, to.y); y++) {
        for (var x = min(from.x, to.x); x <= max(from.x, to.x); x++) {
          map[Point(x, y)] = "#";
        }
      }
    }
  }
  return map;
}

class Point {
  final int x;
  final int y;
  Point(this.x, this.y);
  @override
  operator ==(p) => p is Point && p.x == x && p.y == y;
  int get hashCode => Object.hash(x, y);
}
