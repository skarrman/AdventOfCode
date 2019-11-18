#include "../utils/TxtReader.h"
#include <iostream>
#include <math.h>
#include <string>
using namespace std;

struct Coord {
  int x, y;
};

struct PointDiff {
  int id, dist;
};

inline Coord *getCoord(string row) {
  string::size_type sz;
  Coord *c = new Coord;
  size_t comma = row.find(',');
  c->x = stoi(row.substr(0, comma), &sz);
  c->y = stoi(row.substr(comma + 1, row.size() - 1), &sz);
  return c;
}

inline Coord **getCoords(int amount, vector<string> rows) {
  Coord **coords = new Coord *[amount];

  int i = 0;
  for (auto it = rows.begin(); it != rows.end(); ++it, i++) {
    coords[i] = getCoord(*it);
  }
  return coords;
}

inline void firstTask(int inputs, Coord **coords, int width, int height,
                      PointDiff **grid) {
  for (int i = 0; i < inputs; i++) {
    Coord c = *coords[i];
    for (int j = 0; j < width; j++) {
      for (int k = 0; k < height; k++) {
        int dist = abs(c.x - j) + abs(c.y - k);
        if (grid[j][k].dist > dist) {
          grid[j][k].dist = dist;
          grid[j][k].id = i;
        } else if (grid[j][k].dist == dist) {
          grid[j][k].id = -1;
        }
      }
    }
  }

  int max = 0;
  for (int i = 0; i < inputs; i++) {
    int count = 0;
    for (int j = 0; j < width; j++) {
      for (int k = 0; k < height; k++) {
        if (grid[j][k].id == i)
          count += 1;
      }
    }
    if (count > max)
      max = count;
  }
  cout << max << endl;
}

inline void secondTask(int inputs, Coord **coords, int width, int height,
                       PointDiff **grid) {
  for (int j = 0; j < width; j++) {
    for (int k = 0; k < height; k++) {
      int sum = 0;
      for (int i = 0; i < inputs; i++) {
        Coord c = *coords[i];
        sum += abs(c.x - j) + abs(c.y - k);
      }
      if (sum < 10000)
        grid[j][k].id = 1;
    }
  }
  int sum = 0;
  for (int i = 0; i < width; i++) {
    for (int j = 0; j < height; j++) {
      if (grid[i][j].id == 1)
        sum += 1;
    }
  }
  cout << sum << endl;
}

inline PointDiff **reset(PointDiff **grid, int width, int height) {
  for (int i = 0; i < width; i++) {
    for (int j = 0; j < height; j++) {
      PointDiff pf;
      pf.dist = 0x7FFFFFFF;
      pf.id = -1;
      grid[i][j] = pf;
    }
  }
  return grid;
}

int main() {
  string file = "06/input.txt";
  TxtReader reader;
  vector<string> rows = reader.getStringFromFile(file);
  int inputs = reader.getNumberOfRows(file);

  Coord **coords = getCoords(inputs, rows);
  int minX = 0x7FFFFFFF, maxX = 0, minY = 0x7FFFFFFF, maxY = 0;
  for (int i = 0; i < inputs; i++) {
    Coord c = *coords[i];
    if (c.x < minX)
      minX = c.x;
    if (c.x > maxX)
      maxX = c.x;
    if (c.y < minY)
      minY = c.y;
    if (c.y > maxY)
      maxY = c.y;
  }

  for (int i = 0; i < inputs; i++) {
    coords[i]->x -= (minX - 1);
    coords[i]->y -= (minY - 1);
  }

  int width = maxX - minX + 2;
  int height = maxY - minY + 2;

  PointDiff **grid = new PointDiff *[width];
  for (int i = 0; i < width; i++) {
    grid[i] = new PointDiff[height];
  }

  grid = reset(grid, width, height);
  firstTask(inputs, coords, width, height, grid);

  grid = reset(grid, width, height);
  secondTask(inputs, coords, width, height, grid);

  delete[] grid;
  delete[] coords;
}
