#include "../utils/TxtReader.h"
#include <iostream>
#include <string>
#include <vector>

using namespace std;

inline int twoOfAny(string s) {
  for (string::size_type i = 0; i < s.size(); ++i) {
    int times = 0;
    for (string::size_type j = 0; j < s.size(); ++j) {
      if (s[i] == s[j]) {
        times += 1;
      }
    }
    if (times == 2)
      return 1;
  }
  return 0;
}

inline int threeOfAny(string s) {
  for (string::size_type i = 0; i < s.size(); ++i) {
    int times = 0;
    for (string::size_type j = 0; j < s.size(); ++j) {
      if (s[i] == s[j]) {
        times += 1;
      }
    }
    if (times == 3)
      return 1;
  }
  return 0;
}

inline void firstTask(void) {
  TxtReader reader;
  vector<string> rows = reader.getStringFromFile("02/input.txt");
  int two = 0;
  int three = 0;
  for (auto s = rows.begin(); s != rows.end(); ++s) {
    two += twoOfAny(*s);
    three += threeOfAny(*s);
  }
  int checksum = two * three;
  cout << "Checksum: " << checksum << endl;
}

inline int getDiffs(string s1, string s2) {
  int diffs = 0;
  for (size_t i = 0; i < s1.length(); i++) {
    if (s1[i] != s2[i])
      diffs += 1;
  }
  return diffs;
}

inline string removeDiff(string s1, string s2) {
  string s = "";
  for (size_t i = 0; i < s1.size(); i++) {
    if (s1[i] == s2[i])
      s += s1[i];
  }
  return s;
}

inline void secondTask(void) {

  TxtReader reader;
  vector<string> rows = reader.getStringFromFile("02/input.txt");
  string s;

  int i = 0, j;
  for (auto it = rows.begin(); it != rows.end(); ++it, i++) {
    j = 0;
    for (auto jt = rows.begin(); jt != rows.end(); ++jt, j++) {
      if (j < i + 1)
        continue;
      int diffs = getDiffs(*it, *jt);
      if (diffs == 1) {
        s = removeDiff(*it, *jt);
        break;
      }
    }
  }
  cout << "Common letters: " << s << endl;
}

int main() {
  firstTask();
  secondTask();
  return 0;
}
