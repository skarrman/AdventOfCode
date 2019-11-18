#include "TxtReader.h"
#include <fstream>
#include <iostream>

using namespace std;

vector<int> TxtReader::getIntsFromFile(string filename) {
  ifstream input(filename);
  vector<int> rows;

  string inputLine;
  string::size_type sz;

  if (input.is_open()) {
    while (getline(input, inputLine)) {
      rows.push_back(stoi(inputLine, &sz));
    }
    input.close();
  } else {
    cout << "File not found" << endl;
  }
  rows.shrink_to_fit();
  return rows;
}

vector<string> TxtReader::getStringFromFile(string filename) {
  ifstream input(filename);
  vector<string> rows;

  string inputLine;

  if (input.is_open()) {
    while (getline(input, inputLine)) {
      rows.push_back(inputLine);
    }
    input.close();
  } else {
    cout << "File not found" << endl;
  }
  rows.shrink_to_fit();
  return rows;
}
size_t TxtReader::getNumberOfRows(string filename) {
  ifstream input(filename);

  string inputLine;
  size_t number = 0;

  if (input.is_open()) {
    while (getline(input, inputLine)) {
      number += 1;
    }
    input.close();
  } else {
    cout << "File not found" << endl;
  }
  return number;
}
