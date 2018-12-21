#include <iostream>
#include <vector>

using namespace std;

inline void firstTask(vector<size_t> recepies) {
  size_t firstElf = 0;
  size_t secondElf = 1;
  size_t recepiesAmout = 513401;

  while (recepies.size() < recepiesAmout + 10) {
    size_t newRecepies = recepies[firstElf] + recepies[secondElf];
    if (newRecepies > 9)
      recepies.push_back(newRecepies / 10);
    recepies.push_back(newRecepies % 10);

    firstElf = (firstElf + recepies[firstElf] + 1) % recepies.size();
    secondElf = (secondElf + recepies[secondElf] + 1) % recepies.size();
  }

  for (size_t i = recepiesAmout; i < recepiesAmout + 10; i++)
    cout << recepies[i];
  cout << endl;
}

inline void secondTask(vector<size_t> recepies) {
  size_t firstElf = 0;
  size_t secondElf = 1;
  vector<size_t> numberSecuence; // = 513401; 59414 513401
  numberSecuence.push_back(5);
  numberSecuence.push_back(1);
  numberSecuence.push_back(3);
  numberSecuence.push_back(4);
  numberSecuence.push_back(0);
  numberSecuence.push_back(1);
  // numberSecuence.push_back(5);
  // numberSecuence.push_back(9);
  // numberSecuence.push_back(4);
  // numberSecuence.push_back(1);
  // numberSecuence.push_back(4);

  bool match = false;
  size_t secuenceIndex = 0;

  while (!match) {
    size_t newRecepies = recepies[firstElf] + recepies[secondElf];
    if (newRecepies > 9)
      recepies.push_back(newRecepies / 10);
    recepies.push_back(newRecepies % 10);

    firstElf = (firstElf + recepies[firstElf] + 1) % recepies.size();
    secondElf = (secondElf + recepies[secondElf] + 1) % recepies.size();

    if (recepies.size() > numberSecuence.size()) {
      match = true;
      size_t i = 0;
      for (size_t j = recepies.size() - numberSecuence.size();
           j < recepies.size(); j++) {
        if (recepies[j] != numberSecuence[i]) {
          match = false;
          break;
        }
        i++;
      }
      if (match) {
        secuenceIndex = recepies.size() - numberSecuence.size();
        break;
      } else {
        i = 0;
        match = true;
        for (size_t j = recepies.size() - numberSecuence.size() - 1;
             j < recepies.size() - 1; j++) {
          if (recepies[j] != numberSecuence[i]) {
            match = false;
            break;
          }
          i++;
        }
        if (match) {
          secuenceIndex = recepies.size() - numberSecuence.size() - 1;
          break;
        }
      }
    }
  }
  cout << (secuenceIndex) << endl;
}

int main() {
  vector<size_t> recepies;
  recepies.push_back(3);
  recepies.push_back(7);

  //   firstTask(recepies);
  secondTask(recepies);
}
