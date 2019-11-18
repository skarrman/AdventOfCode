#include "../utils/TxtReader.h"
#include "date.h"
#include <fstream>
#include <iostream>
#include <list>
#include <map>
using namespace std;

inline string getDateString(string line) {
  size_t start = line.find('[');
  size_t end = line.find(']');
  return line.substr(start + 1, end - 1);
}

inline int getGuardId(string line) {
  size_t start = line.find('#');

  string s = "";
  for (size_t i = start + 1; i < line.length(); ++i) {
    if (line[i] == ' ') {
      break;
    } else
      s += line[i];
  }
  string::size_type sz;
  return stoi(s, &sz);
}

inline int getTotalHour(int *hours) {
  int sum = 0;
  for (int i = 0; i < 60; i++) {
    sum += hours[i];
  }
  return sum;
}

inline int getHour(int *hours) {
  int max = -1;
  int index = -1;
  for (int i = 0; i < 60; i++) {
    if (max < hours[i]) {
      max = hours[i];
      index = i;
    }
  }
  return max;
}

inline int getMaxHour(int *hours) {
  int max = -1;
  int index = -1;
  for (int i = 0; i < 60; i++) {
    if (max < hours[i]) {
      max = hours[i];
      index = i;
    }
  }
  return index;
}

int main() {
  string file = "04/sortedInput.txt";
  TxtReader reader;
  vector<string> rows = reader.getStringFromFile(file);
  map<int, int *> guardHours;

  for (auto it = rows.begin(); it != rows.end(); ++it) {
    int guardId = getGuardId(*it);
    if (guardHours.find(guardId) == guardHours.end())
      guardHours.insert({guardId, new int[60]});
    while ((++it)->find('#') == string::npos && it != rows.end()) {
      // Add minites to hour list
      Date startDate = Date(getDateString(*it));
      Date endDate = Date(getDateString(*(++it)));

      for (int i = startDate.minute; i < endDate.minute; i++)
        guardHours[guardId][i] += 1;
    }
    it--;
  }
  int maxTime = 0;
  int maxTimeGuard = 0;
  for (auto it = guardHours.begin(); it != guardHours.end(); it++) {
    int t = getTotalHour(it->second);

    if (maxTime < t) {
      maxTime = t;
      maxTimeGuard = it->first;
    }
  }
  int maxFrequentHour = getMaxHour(guardHours[maxTimeGuard]);
  cout << maxTimeGuard * maxFrequentHour << endl;
  ;

  int mostOccuringHour = 0;
  int mostOccuringGuard = 0;
  int index = 0;
  for (auto it = guardHours.begin(); it != guardHours.end(); it++) {
    int t = getHour(it->second);

    if (mostOccuringHour < t) {
      mostOccuringHour = t;
      mostOccuringGuard = it->first;
      index = getMaxHour(it->second);
    }
  }
  cout << mostOccuringGuard * index << endl;
  return 0;
}
