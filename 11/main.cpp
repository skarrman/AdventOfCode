#include "../utils/TxtReader.h"
#include <iostream>
#include <string>
#include <vector>

using namespace std;

inline int getHundred(int num) {
  num = num % 1000;
  num /= 100;
  return num;
}

inline int getPowerLevel(int x, int y, int serialNumber) {
  int rackID = x + 10;
  int powerLevel = rackID * y;
  powerLevel += serialNumber;
  powerLevel *= rackID;
  powerLevel = getHundred(powerLevel);
  return powerLevel - 5;
}

int main() {

  int serialNumber = 9798;
  int **grid = new int *[300];
  for (int i = 0; i < 300; i++)
    grid[i] = new int[300];

  for (int i = 0; i < 300; i++) {
    for (int j = 0; j < 300; j++) {
      grid[i][j] = getPowerLevel(i, j, serialNumber);
    }
  }

  int gridSize = 3;
  int max = 0, maxX = 0, maxY = 0, maxGridSize = 0;
  for (gridSize = 0; gridSize < 300; gridSize++) {
    for (int i = 0; i <= 300 - gridSize; i++) {
      for (int j = 0; j <= 300 - gridSize; j++) {
        int sum = 0;
        for (int k = i; k < i + gridSize; k++) {
          for (int l = j; l < j + gridSize; l++) {
            sum += grid[k][l];
          }
        }
        if (sum > max) {
          max = sum;
          maxX = i;
          maxY = j;
          maxGridSize = gridSize;
        }
      }
    }
  }

  cout << maxX << "," << maxY << "," << maxGridSize << " " << max << endl;

  return 0;
}
