#include <iostream>
#include <string>
#include <vector>
using namespace std;

inline unsigned long getMaxScore(unsigned long *score, unsigned long players) {
  unsigned long max = 0;
  for (unsigned long i = 0; i < players; i++) {
    if (score[i] > max)
      max = score[i];
  }
  return max;
}

inline void printRing(vector<unsigned long> ring) {
  for (unsigned long l : ring) {
    cout << l << " ";
  }
  cout << endl;
}

inline void firstTask(unsigned long players, unsigned long highestMarble) {
  unsigned long nextMarble = 0;
  unsigned long currentMarble = 0;
  unsigned long turn = 0;
  unsigned long *score = new unsigned long[players];
  vector<unsigned long> ring;

  for (unsigned long i = 0; i < players; i++)
    score[i] = 0;

  ring.push_back(0);
  ring.push_back(1);
  nextMarble = 2;
  turn = 1;
  currentMarble = 2;

  while (nextMarble <= highestMarble) {
    // printRing(ring);
    if (nextMarble % 23 == 0) {
      score[turn] += nextMarble;
      unsigned long toRemove = (currentMarble - 7) % ring.size();
      score[turn] += ring[toRemove];
      ring.erase(ring.begin() + toRemove);
      currentMarble = toRemove;
    } else {
      unsigned long index = (currentMarble + 2) % ring.size();
      if (index == 0)
        index = ring.size();
      ring.insert(ring.begin() + index, nextMarble);
      currentMarble = index;
    }
    nextMarble += 1;
    turn = (turn + 1) % players;
  }

  cout << "Max score: " << getMaxScore(score, players) << endl;
}

int main() {
  // Test
  // firstTask(9, 25);
  // firstTask(10, 1618);
  // firstTask(13, 7999);
  // firstTask(17, 1104);
  // firstTask(21, 6111);
  // firstTask(30, 5807);
  // First task
  firstTask(473, 70904);
  // Second task
  // firstTask(473, 70904 * 100);
  return 0;
}
