#include "../utils/TxtReader.h"
#include <iostream>
#include <string>
#include <vector>

using namespace std;

inline char **newMap(int height, int width) {
  char **map = new char *[height];
  for (int i = 0; i < height; i++) {
    map[i] = new char[width];
  }
  return map;
}

inline bool isValid(int y, int x, int height, int width) {
  return y >= 0 && y < height && x >= 0 && x < width;
}

inline char **update(char **old, int height, int width) {
  char **map = newMap(height, width);
  int yNum[] = {0, -1, -1, -1, 0, 1, 1, 1};
  int xNum[] = {-1, -1, 0, 1, 1, 1, 0, -1};
  for (int i = 0; i < height; i++) {
    for (int j = 0; j < width; j++) {
      if (old[i][j] == '.') {
        int sum = 0;
        for (int k = 0; k < 8; k++) {
          if (isValid(i + yNum[k], j + xNum[k], height, width)) {
            if (old[i + yNum[k]][j + xNum[k]] == '|')
              sum += 1;
          }
        }
        if (sum > 2)
          map[i][j] = '|';
        else
          map[i][j] = '.';
      } else if (old[i][j] == '|') {
        int sum = 0;
        for (int k = 0; k < 8; k++) {
          if (isValid(i + yNum[k], j + xNum[k], height, width)) {
            if (old[i + yNum[k]][j + xNum[k]] == '#')
              sum += 1;
          }
        }
        if (sum > 2)
          map[i][j] = '#';
        else
          map[i][j] = '|';
      } else {
        int lumb = 0;
        int trees = 0;
        for (int k = 0; k < 8; k++) {
          if (isValid(i + yNum[k], j + xNum[k], height, width)) {
            if (old[i + yNum[k]][j + xNum[k]] == '#')
              lumb += 1;
            else if (old[i + yNum[k]][j + xNum[k]] == '|')
              trees += 1;
          }
        }
        if (lumb > 0 && trees > 0)
          map[i][j] = '#';
        else
          map[i][j] = '.';
      }
    }
  }
  delete[] old;
  return map;
}

inline void printMap(char **map, int height, int width) {
  for (int i = 0; i < height; i++) {
    for (int j = 0; j < width; j++) {
      cout << map[i][j];
    }
    cout << endl;
  }
  cout << endl;
}
inline int getNumberOf(char c, char **map, int height, int width) {
  int sum = 0;
  for (int i = 0; i < height; i++) {
    for (int j = 0; j < width; j++) {
      if (map[i][j] == c)
        sum += 1;
    }
  }
  return sum;
}

int main() {
  TxtReader reader;
  vector<string> rows = reader.getStringFromFile("18/input.txt");
  int height = rows.size();
  int width = rows[0].size();

  char **map = newMap(height, width);
  for (int i = 0; i < height; i++) {
    for (int j = 0; j < width; j++) {
      map[i][j] = rows[i][j];
    }
  }

  // 1000000000
  for (int i = 0; i < 1000; i++) {
    map = update(map, height, width);
  }
  int trees = getNumberOf('|', map, height, width);
  int lumber = getNumberOf('#', map, height, width);

  cout << (trees * lumber) << endl;
}
