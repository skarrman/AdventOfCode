#include "../utils/TxtReader.h"
#include <iostream>
#include <vector>
using namespace std;

#define GRID_SIZE 1000

struct Area {
  int id, startY, startX, width, height;
};

inline void dispose(vector<Area *> areas) {
  for (auto it = areas.begin(); it != areas.end(); ++it)
    delete (*it);
}

inline Area *parse(string line) {
  Area *area = new Area;
  string number = "";
  size_t tag = line.find('#');
  string::size_type sz;
  for (size_t i = tag + 1; i < line.length(); i++) {
    if (isspace(line[i])) {
      area->id = stoi(number, &sz);
      number = "";
      i += 2;
    } else if (line[i] == ',') {
      area->startY = stoi(number, &sz);
      number = "";
    } else if (line[i] == ':') {
      area->startX = stoi(number, &sz);
      number = "";
      i += 1;
    } else if (line[i] == 'x') {
      area->height = stoi(number, &sz);
      number = "";
    } else {
      number += line[i];
    }
  }
  area->width = stoi(number, &sz);
  return area;
}

inline vector<Area *> getAreasFrom(vector<string> rows) {
  vector<Area *> areas;
  for (auto it = rows.begin(); it != rows.end(); ++it) {
    areas.push_back(parse(*it));
  }
  return areas;
}

inline void firstTask(void) {
  TxtReader reader;
  vector<string> rows = reader.getStringFromFile("03/input.txt");
  vector<Area *> areas = getAreasFrom(rows);

  int **grid = new int *[GRID_SIZE];
  for (int i = 0; i < GRID_SIZE; i++)
    grid[i] = new int[GRID_SIZE];

  int sum = 0;
  for (auto it = areas.begin(); it != areas.end(); ++it) {
    for (int i = (*it)->startX; i < (*it)->startX + (*it)->width; i++) {
      for (int j = (*it)->startY; j < (*it)->startY + (*it)->height; j++) {
        grid[i][j] += 1;
        if (grid[i][j] == 2)
          sum += 1;
      }
    }
  }
  cout << "Overlapping squares: " << sum << endl;
  delete[] grid;
  dispose(areas);
}

inline void secondTask(void) {
  TxtReader reader;
  vector<string> rows = reader.getStringFromFile("03/input.txt");
  vector<Area *> areas = getAreasFrom(rows);

  int **grid = new int *[GRID_SIZE];
  for (int i = 0; i < GRID_SIZE; i++)
    grid[i] = new int[GRID_SIZE];

  for (auto it = areas.begin(); it != areas.end(); ++it) {
    for (int i = (*it)->startX; i < (*it)->startX + (*it)->width; i++) {
      for (int j = (*it)->startY; j < (*it)->startY + (*it)->height; j++) {
        grid[i][j] += 1;
      }
    }
  }
  int id = -1;
  for (auto it = areas.begin(); it != areas.end(); ++it) {
    bool isOverLap = false;
    for (int i = (*it)->startX; i < (*it)->startX + (*it)->width; i++) {
      for (int j = (*it)->startY; j < (*it)->startY + (*it)->height; j++) {
        if (grid[i][j] > 1)
          isOverLap = true;
      }
    }
    if (!isOverLap) {
      id = (*it)->id;
      break;
    }
  }

  cout << "Now overlapping id: " << id << endl;
  delete[] grid;
  dispose(areas);
}

int main() {
  firstTask();
  secondTask();
  return 0;
}
