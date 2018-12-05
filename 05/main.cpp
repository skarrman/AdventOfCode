#include "../utils/TxtReader.h"
#include <iostream>
#include <string>
using namespace std;

inline string removePolymer(string row) {
  string::size_type first = 0;
  char last = row[first];
  string remaningRow = "";
  remaningRow.push_back(last);
  for (string::size_type i = 1; i < row.size(); i++) {
    if (last == row[i] - 32 || last == row[i] + 32) {
      remaningRow.pop_back();
      //   char lastLast = last;
      last = remaningRow[remaningRow.size() - 1];
      //   cout << lastLast << row[i] << last << endl;
    } else {
      remaningRow.push_back(row[i]);
      last = row[i];
    }
  }
  return remaningRow;
}

inline void secondTask(string row) {
  string letters = "abcdefghijklmnopqrstuvwxyz";
  string::size_type min = 0xFFFFFFFF;
  for (string::size_type l = 0; l < letters.size(); l++) {
    string removedRow = "";
    for (string::size_type i = 0; i < row.size(); i++) {
      if (row[i] != letters[l] && row[i] != letters[l] - 32)
        removedRow.push_back(row[i]);
    }
    removedRow = removePolymer(removedRow);
    string::size_type length = removedRow.size();
    if (length < min)
      min = length;
  }
  cout << min << endl;
}

int main() {
  TxtReader reader;
  string row = (reader.getStringFromFile("05/input.txt"))[0];
  string newRow = removePolymer(row);
  cout << newRow.length() << endl;
  secondTask(row);
}
