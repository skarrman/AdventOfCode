#include "../utils/TxtReader.h"
#include <iostream>
#include <string>
#include <vector>

using namespace std;

struct point {
  int x, y;
  bool operator==(const point p) { return p.x == x && p.y == y; }
};
void fill(char **, point, int);

inline vector<point> getPointsFromRow(string row) {
  vector<point> points;
  string::size_type sz;
  int coord = stoi(
      row.substr(row.find_first_of('=') + 1, row.find_first_of(',') - 1), &sz);
  int start, end;
  start = stoi(
      row.substr(row.find_last_of('=') + 1, row.find_first_of('.') - 1), &sz);
  end = stoi(row.substr(row.find_last_of('.') + 1, row.find_last_of(',') - 1),
             &sz);

  for (int i = start; i <= end; i++) {
    if (row[0] == 'x') {
      points.push_back({coord, i});
    } else {
      points.push_back({i, coord});
    }
  }
  return points;
}

inline vector<point> getPoints(vector<string> rows) {
  vector<point> points;
  for (string row : rows) {
    for (point p : getPointsFromRow(row))
      points.push_back(p);
  }
  return points;
}

inline int getMinX(vector<point> points) {
  int min = INT32_MAX;
  for (point p : points) {
    if (p.x < min)
      min = p.x;
  }
  return min;
}
inline int getMinY(vector<point> points) {
  int min = INT32_MAX;
  for (point p : points) {
    if (p.y < min)
      min = p.y;
  }
  return min;
}

inline int getMaxX(vector<point> points) {
  int max = INT32_MIN;
  for (point p : points) {
    if (p.x > max)
      max = p.x;
  }
  return max;
}
inline int getMaxY(vector<point> points) {
  int max = INT32_MIN;
  for (point p : points) {
    if (p.y > max)
      max = p.y;
  }
  return max;
}

inline vector<point> normalize(vector<point> points, int x) {
  for (auto it = points.begin(); it != points.end(); ++it)
    (*it).x -= x;
  return points;
}

inline void printGround(char **ground, int height, int width) {
  for (int i = 0; i < height; i++) {
    for (int j = 0; j < width; j++) {
      cout << ground[i][j];
    }
    cout << endl;
  }
}

inline int count(char **ground, int width, int height) {
  int sum = 0;
  for (int i = 0; i < height; i++) {
    for (int j = 0; j < width; j++) {
      if (ground[i][j] == '~' || ground[i][j] == '|')
        sum += 1;
    }
  }
  return sum;
}

inline bool update(char **ground, int height, int width) {
  bool didUpdate = false;
  for (int i = 0; i < height - 1; i++) {
    for (int j = 1; j < width; j++) {
      if (ground[i][j] == '|') {
        if (ground[i + 1][j] == '.') {
          ground[i + 1][j] = '|';
          didUpdate = true;
        } else if (ground[i + 1][j] == '#') {
          ground[i][j] = '~';
          didUpdate = true;
        }
      }
      if (ground[i][j] == '~') {
        if (ground[i][j + 1] == '.' && ground[i + 1][j + 1] != '.') {
          ground[i][j + 1] = '~';
          didUpdate = true;
        }
        if (ground[i][j - 1] == '.' && ground[i + 1][j - 1] != '.') {
          ground[i][j - 1] = '~';
          didUpdate = true;
        }

        if ((ground[i + 1][j - 1] == '~' || ground[i + 1][j - 1] == '#') &&
            (ground[i + 1][j + 1] == '~' || ground[i + 1][j + 1] == '#')) {
          ground[i - 1][j] = '~';
          didUpdate = true;
        }
      }
    }
  }
  return didUpdate;
}

// 232 1771 565 1771

int main() {
  TxtReader reader;
  vector<string> rows = reader.getStringFromFile("17/test.txt");
  vector<point> points = getPoints(rows);
  int minX = getMinX(points);
  points = normalize(points, minX - 1);
  int width = getMaxX(points) + 2;
  int height = getMaxY(points) + 1;
  cout << width << " " << height << endl;
  char **ground = new char *[height];
  point waterSpring = {500 - minX + 1, 0};
  for (int i = 0; i < height; i++) {
    ground[i] = new char[width];
    for (int j = 0; j < width; j++)
      ground[i][j] = '.';
  }
  // Width constrains

  ground[waterSpring.y][waterSpring.x] = '+';
  ground[waterSpring.y + 1][waterSpring.x] = '|';
  for (point p : points)
    ground[p.y][p.x] = '#';
  // printGround(ground, height, width);
  update(ground, height, width);
  update(ground, height, width);
  update(ground, height, width);
  update(ground, height, width);
  update(ground, height, width);
  update(ground, height, width);

  printGround(ground, height, width);
  cout << count(ground, width, height) << endl;
  delete[] ground;
  return 0;
}
