#include "../utils/TxtReader.h"
#include <iostream>
#include <map>
#include <string>
#include <vector>

using namespace std;

inline int intValue(char c) { return c == '#' ? 1 : 0; }

inline vector<int> getInitialState(string row) {
  vector<int> state;
  auto it = state.begin();
  state.insert(it, 3, 0);
  for (char c : row)
    state.push_back(intValue(c));
  it = state.end();
  state.insert(it, 3, 0);
  return state;
}

inline int ruleHash(int n1, int n2, int n3, int n4, int n5) {
  return n1 * 13 + n2 * 27 + n3 * 37 + n4 * 53 + n5 * 61;
}

inline map<int, int> getRules(vector<string> rows) {
  map<int, int> rules;
  size_t i = 0;

  for (string s : rows)
    rules.insert(
        {ruleHash(intValue(s[i]), intValue(s[i + 1]), intValue(s[i + 2]),
                  intValue(s[i + 3]), intValue(s[i + 4])),
         intValue(s[i + 9])});

  return rules;
}

inline int getPlats(vector<int> state, int offset) {
  int sum = 0;
  for (int i = 0; i < state.size(); i++) {
    if (state[i] == 1) {
      sum += (i - offset);
    }
  }
  return sum;
}

int main() {
  TxtReader reader;
  vector<string> rows = reader.getStringFromFile("12/input.txt");
  vector<int> state = getInitialState(rows[0]);

  auto it = rows.begin();
  rows.erase(it);
  unsigned long offset = 3;
  map<int, int> rules = getRules(rows);
  for (long gen = 0; gen < 50000; gen++) {
    vector<int> newState;
    for (int k = 0; k < 3; k++)
      newState.push_back(0);
    unsigned long newOffset = 0;
    for (unsigned long i = 2; i < state.size() - 2; i++) {
      int n = rules[ruleHash(state[i - 2], state[i - 1], state[i], state[i + 1],
                             state[i + 2])];
      newState.push_back(n);
      if (i == offset)
        newOffset = newState.size() - 1;
    }
    offset = newOffset;
    for (int k = 0; k < 3; k++)
      newState.push_back(0);
    state = newState;
  }

  cout << getPlats(state, offset) << endl;

  // 5000 -> 432414 and 50000 -> 4302414 => 500000000000 -> 4300000002414

  return 0;
}
