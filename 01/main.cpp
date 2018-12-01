#include <fstream>
#include <iostream>
#include <string>
#define LENGTH 993
#define LAPS 1000000
using namespace std;

void firstTask() {
  string inputLine;
  int res = 0;
  ifstream input("01/input.txt");

  string::size_type sz;

  if (input.is_open()) {
    while (getline(input, inputLine)) {
      int value = stoi(inputLine, &sz);
      res += value;
    }
    input.close();
  } else
    cout << "File not open" << endl;

  cout << res << endl;
}

bool hasOccured(int *list, int length, int value) {
  for (int i = 0; i < length; i++) {
    if (list[i] == value)
      return true;
  }
  return false;
}

void secondTask() {
  int *inputs = new int[LENGTH];
  int *results = new int[LENGTH * LAPS];

  int inputIndex = 0;

  ifstream input("01/input.txt");

  string inputLine;
  string::size_type sz;

  if (input.is_open()) {
    while (getline(input, inputLine)) {
      int value = stoi(inputLine, &sz);
      inputs[inputIndex++] = value;
    }
    input.close();

    int result = 0;
    results[0] = result;
    inputIndex = 0;
    int index = 1;
    for (int i = 0; i < LAPS; i++) {
      result += inputs[inputIndex];
      inputIndex = (inputIndex + 1) % LENGTH;
      if (hasOccured(results, index, result)) {
        cout << "Twice: " << result << endl;
        break;
      } else {
        results[index] = result;
        index += 1;
      }
    }

  } else
    cout << "File not open" << endl;

  delete[] results;
  delete[] inputs;
}

int main() {
  firstTask();
  secondTask();
  return 0;
}
