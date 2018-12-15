#include "../utils/TxtReader.h"
#include <iostream>
#include <string>
#include <vector>

using namespace std;

struct direction {
  int x, y;
  direction() : x(0), y(0) {}
  direction(int i, int j) {
    x = i;
    y = j;
  }
  bool operator==(const direction d) { return d.x == x && d.y == y; }
};

struct cart {
  int id;
  direction dir;
  int nextInterSection = 0;
  cart() : id(-1) {}
};

inline void printMap(cart **carts, char **paths, size_t height, size_t width) {
  for (size_t i = 0; i < height; i++) {
    for (size_t j = 0; j < width; j++) {
      char c = paths[i][j];
      if (carts[i][j].id != -1) {
        direction d = carts[i][j].dir;
        if (d.x == -1 && d.y == 0)
          c = '^';
        else if (d.x == 1 && d.y == 0)
          c = 'v';
        else if (d.x == 0 && d.y == -1)
          c = '<';
        else if (d.x == 0 && d.y == 1)
          c = '>';
      }
      cout << c;
    }
    cout << endl;
  }
}

inline void localize(cart **carts, size_t height, size_t width) {
  for (size_t i = 0; i < height; i++) {
    for (size_t j = 0; j < width; j++) {
      if (carts[i][j].id != -1) {
        cout << "ID: " << carts[i][j].id << " " << j << "," << i << endl;
      }
    }
  }
}

int main() {
  TxtReader reader;
  string file = "13/input.txt";
  vector<string> rows = reader.getStringFromFile(file);
  size_t height = reader.getNumberOfRows(file);

  const direction UP = direction(-1, 0);
  const direction DOWN = direction(1, 0);
  const direction RIGHT = direction(0, 1);
  const direction LEFT = direction(0, -1);

  size_t width = 0;
  for (string row : rows) {
    if (row.size() > width)
      width = row.size();
  }

  char **paths = new char *[height];
  cart **carts = new cart *[height];
  for (size_t i = 0; i < height; i++) {
    paths[i] = new char[width];
    carts[i] = new cart[width];
  }

  int id = 0;
  for (size_t i = 0; i < height; i++) {
    for (size_t j = 0; j < width; j++) {
      cart c;
      switch (rows[i][j]) {
      case '>':
        c.id = id++;
        c.dir = RIGHT;
        paths[i][j] = '-';
        carts[i][j] = c;
        break;
      case '<':
        c.id = id++;
        c.dir = LEFT;
        paths[i][j] = '-';
        carts[i][j] = c;
        break;
      case '^':
        c.id = id++;
        c.dir = UP;
        paths[i][j] = '|';
        carts[i][j] = c;
        break;
      case 'v':
        c.id = id++;
        c.dir = DOWN;
        paths[i][j] = '|';
        carts[i][j] = c;
        break;
      default:
        carts[i][j].id = -1;
        paths[i][j] = rows[i][j];
      }
    }
  }

  //   printMap(carts, paths, height, width);

  bool colision = false;
  vector<int> updated;
  updated.push_back(0);
  updated.push_back(1);
  size_t x = 0, y = 0;
  while (updated.size() > 1) {
    updated.clear();
    for (size_t i = 0; i < height; i++) {

      for (size_t j = 0; j < width; j++) {
        if (carts[i][j].id != -1) {
          cart c = carts[i][j];
          if (find(updated.begin(), updated.end(), c.id) == updated.end()) {
            updated.push_back(c.id);
            x = j;
            y = i;
            if (c.dir == UP) {
              if (carts[i - 1][j].id != -1) {
                colision = true;
                x = j;
                y = i - 1;
                c.id = -1;
                carts[i][j].id = -1;
                carts[i - 1][j].id = -1;
                continue;
              }
              switch (paths[i - 1][j]) {
              case '/':
                c.dir = RIGHT;
                break;
              case '\\':
                c.dir = LEFT;
                break;
              case '+': {
                switch (c.nextInterSection) {
                case 0:
                  c.dir = LEFT;
                  c.nextInterSection += 1;
                  break;
                case 1:
                  c.dir = UP;
                  c.nextInterSection += 1;
                  break;
                case 2:
                  c.dir = RIGHT;
                  c.nextInterSection = 0;
                  break;
                }
              } break;
              }
              carts[i - 1][j].id = c.id;
              carts[i - 1][j].dir = c.dir;
              carts[i - 1][j].nextInterSection = c.nextInterSection;
              carts[i][j].id = -1;
            } else if (c.dir == DOWN) {
              if (carts[i + 1][j].id != -1) {
                colision = true;
                x = j;
                y = i + 1;
                c.id = -1;
                carts[i][j].id = -1;
                carts[i + 1][j].id = -1;
                continue;
              }
              switch (paths[i + 1][j]) {
              case '/':
                c.dir = LEFT;
                break;
              case '\\':
                c.dir = RIGHT;
                break;
              case '+': {
                switch (c.nextInterSection) {
                case 0:
                  c.dir = RIGHT;
                  c.nextInterSection += 1;
                  break;
                case 1:
                  c.dir = DOWN;
                  c.nextInterSection += 1;
                  break;
                case 2:
                  c.dir = LEFT;
                  c.nextInterSection = 0;
                  break;
                }
              } break;
              }
              carts[i + 1][j].id = c.id;
              carts[i + 1][j].dir = c.dir;
              carts[i + 1][j].nextInterSection = c.nextInterSection;
              carts[i][j].id = -1;
            } else if (c.dir == RIGHT) {
              if (carts[i][j + 1].id != -1) {
                colision = true;
                x = j + 1;
                y = i;
                c.id = -1;
                carts[i][j].id = -1;
                carts[i][j + 1].id = -1;
                continue;
              }
              switch (paths[i][j + 1]) {
              case '\\':
                c.dir = DOWN;
                break;
              case '/':
                c.dir = UP;
                break;
              case '+': {
                switch (c.nextInterSection) {
                case 0:
                  c.dir = UP;
                  c.nextInterSection += 1;
                  break;
                case 1:
                  c.dir = RIGHT;
                  c.nextInterSection += 1;
                  break;
                case 2:
                  c.dir = DOWN;
                  c.nextInterSection = 0;
                  break;
                }
              } break;
              }
              carts[i][j + 1].id = c.id;
              carts[i][j + 1].dir = c.dir;
              carts[i][j + 1].nextInterSection = c.nextInterSection;
              carts[i][j].id = -1;
            } else {
              if (carts[i][j - 1].id != -1) {
                colision = true;
                x = j - 1;
                y = i;
                c.id = -1;
                carts[i][j].id = -1;
                carts[i][j - 1].id = -1;
                continue;
              }
              switch (paths[i][j - 1]) {
              case '/':
                c.dir = DOWN;
                break;
              case '\\':
                c.dir = UP;
                break;
              case '+': {
                switch (c.nextInterSection) {
                case 0:
                  c.dir = DOWN;
                  c.nextInterSection += 1;
                  break;
                case 1:
                  c.dir = LEFT;
                  c.nextInterSection += 1;
                  break;
                case 2:
                  c.dir = UP;
                  c.nextInterSection = 0;
                  break;
                }
              } break;
              }
              carts[i][j - 1].id = c.id;
              carts[i][j - 1].dir = c.dir;
              carts[i][j - 1].nextInterSection = c.nextInterSection;
              carts[i][j].id = -1;
            }
          }
        }
      }
    }
  }
  localize(carts, height, width);
  cout << x << "," << y << endl;
  return 0;
}
