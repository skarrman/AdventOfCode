#include "../utils/TxtReader.h"
#include <algorithm>
#include <functional>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <string>
#include <vector>
using namespace std;

inline map<char, list<char>> putChars(vector<string> rows) {
  map<char, list<char>> chars;
  for (auto it = rows.begin(); it != rows.end(); ++it) {
    char key = it->at(5);
    char value = it->at(36);
    if (chars.find(key) == chars.end()) {
      list<char> l;
      chars.insert(pair<char, list<char>>(key, l));
    }
    chars[key].push_back(value);
  }
  return chars;
}

inline map<char, list<char>> getRestrictions(vector<string> rows) {
  map<char, list<char>> chars;
  for (auto it = rows.begin(); it != rows.end(); ++it) {
    char key = it->at(36);
    char value = it->at(5);
    if (chars.find(key) == chars.end()) {
      list<char> l;
      chars.insert(pair<char, list<char>>(key, l));
    }
    chars[key].push_back(value);
  }
  return chars;
}

inline void firstTask(priority_queue<char, vector<char>, greater<char>> queue,
                      map<char, list<char>> restrictions,
                      map<char, list<char>> enablers) {
  string order = "";

  while (!queue.empty()) {
    char c = queue.top();
    order.push_back(c);
    queue.pop();
    for (auto it = enablers[c].begin(); it != enablers[c].end(); ++it) {
      bool canPut = true;
      for (auto jt = restrictions[*it].begin(); jt != restrictions[*it].end();
           ++jt) {
        if (order.find(*jt) == string::npos)
          canPut = false;
      }
      if (canPut)
        queue.push(*it);
    }
  }
  cout << order << endl;
}

inline bool haveTasks(int *status, int workers) {
  bool haveTask = false;
  for (int i = 0; i < workers; i++) {
    if (status[i] != 0)
      haveTask = true;
  }
  return haveTask;
}

inline void secondTask(priority_queue<char, vector<char>, greater<char>> queue,
                       map<char, list<char>> restrictions,
                       map<char, list<char>> enablers) {
  char extraSeconds = 60;
  int workers = 5;
  int *status = new int[workers];
  char *task = new char[workers];
  int t = 0;
  string order = "";
  while (!queue.empty() || haveTasks(status, workers)) {
    for (int i = 0; i < workers; i++) {
      if (status[i] == 0 && !queue.empty()) {
        char c = queue.top();
        queue.pop();
        task[i] = c;
        status[i] = extraSeconds + (c - 64);
      }
    }
    t += 1;
    for (int i = 0; i < workers; i++) {
      if (status[i] == 1) {
        char c = task[i];
        order.push_back(c);
        for (auto it = enablers[c].begin(); it != enablers[c].end(); ++it) {
          bool canPut = true;
          for (auto jt = restrictions[*it].begin();
               jt != restrictions[*it].end(); ++jt) {
            if (order.find(*jt) == string::npos)
              canPut = false;
          }
          if (canPut)
            queue.push(*it);
        }
      }
      status[i] = status[i] > 0 ? status[i] - 1 : 0;
    }
  }
  cout << order << endl;
  cout << t << endl;
}

int main() {
  string file = "07/input.txt";
  TxtReader reader;
  vector<string> rows = reader.getStringFromFile(file);

  map<char, list<char>> enablers = putChars(rows);

  map<char, list<char>> restrictions = getRestrictions(rows);

  priority_queue<char, vector<char>, greater<char>> queue1;
  priority_queue<char, vector<char>, greater<char>> queue2;

  for (auto it = enablers.begin(); it != enablers.end(); ++it) {
    if (restrictions.find(it->first) == restrictions.end()) {
      queue1.push(it->first);
      queue2.push(it->first);
    }
  }

  //   firstTask(queue1, restrictions, enablers);
  secondTask(queue2, restrictions, enablers);

  return 0;
}
