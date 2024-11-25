import 'dart:collection';
import 'dart:io';
import 'dart:convert';
import 'dart:async';

int solutionPart1(List<List<int>> input) {
  int sum = 0;
  for (int i = 0; i < input.length; i++) {
    for (int j = 0; j < input[i].length; j++) {
      int num = input[i][j];
      if ((i < 1 || input[i - 1][j] > num) &&
          (i == input.length - 1 || input[i + 1][j] > num) &&
          (j < 1 || input[i][j - 1] > num) &&
          (j == input[i].length - 1 || input[i][j + 1] > num)) {
        sum += num + 1;
      }
    }
  }
  return sum;
}

class Pos {
  int i, j;
  Pos(int i, int j) {
    this.i = i;
    this.j = j;
  }
  bool operator ==(Object other) =>
      other is Pos && i == other.i && j == other.j;

  int get hashCode => i * 4027 + j * 6027;
}

List<Pos> search(Pos p, List<List<int>> input) {
  List<Pos> basins = [];
  int num = input[p.i][p.j];
  if (p.i > 0 && input[p.i - 1][p.j] > num && input[p.i - 1][p.j] != 9)
    basins.add(Pos(p.i - 1, p.j));
  if (p.i < input.length - 1 &&
      input[p.i + 1][p.j] > num &&
      input[p.i + 1][p.j] != 9) basins.add(Pos(p.i + 1, p.j));
  if (p.j > 0 && input[p.i][p.j - 1] > num && input[p.i][p.j - 1] != 9)
    basins.add(Pos(p.i, p.j - 1));
  if (p.j < input[p.i].length - 1 &&
      input[p.i][p.j + 1] > num &&
      input[p.i][p.j + 1] != 9) basins.add(Pos(p.i, p.j + 1));
  return basins;
}

int solutionPart2(List<List<int>> input) {
  List<Pos> searched = [];
  HashMap<Pos, Set<Pos>> basinsMap = HashMap();
  for (int i = 0; i < input.length; i++) {
    for (int j = 0; j < input[i].length; j++) {
      List<Pos> toSearch = [];
      toSearch.add(Pos(i, j));
      Set<Pos> basinsSet = Set();
      while (toSearch.isNotEmpty) {
        Pos next = toSearch.removeAt(0);
        if (basinsMap.containsKey(next)) {
          basinsSet.addAll(basinsMap[next]);
          continue;
        } else if (searched.contains(next)) continue;
        var basins = search(next, input);
        basinsSet.addAll(basins);
        toSearch.addAll(basins);
        searched.add(next);
      }
      basinsMap[Pos(i, j)] = basinsSet;
    }
  }
  var sizes = basinsMap.values
      .map((e) => e.length)
      .where((element) => element > 0)
      .map((e) => e + 1)
      .toList()
    ..sort();
  return sizes.reversed
      .take(3)
      .map((e) => e)
      .fold(1, (previousValue, element) => previousValue * element);
}

void main() async {
  print((Platform.environment["part"] ?? "part1") == "part2"
      ? solutionPart2(await parseInput("input.txt"))
      : solutionPart1(await parseInput("input.txt")));
}

Future<List<List<int>>> parseInput(String path) async {
  return new File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(const LineSplitter())
      .map((line) => line.split("").map((digit) => int.parse(digit)).toList())
      .toList();
}
