#include "../utils/TxtReader.h"
#include <algorithm>
#include <iostream>
#include <list>
#include <queue>
#include <string>
#include <vector>

using namespace std;

enum team { GOBLIN, ELF };

struct position {
  int x, y;
  position() : x(0), y(0) {}
  position(int j, int i) {
    x = j;
    y = i;
  }
  bool operator<(const position p) const {
    if (y < p.y)
      return true;
    else if (y == p.y)
      return x < p.x;
    else
      return false;
  }
  bool operator==(const position p) { return x == p.x && y == p.y; }
};

struct unit {
  team t;
  int power, health;
  position pos;

  unit() : power(3), health(200) {}
  bool operator<(const unit u) const { return pos < u.pos; }
};

inline list<unit *> getUnits(char **map, size_t height, size_t width,
                             int elvPower) {
  list<unit *> units;
  for (size_t i = 0; i < height; i++) {
    for (size_t j = 0; j < width; j++) {
      if (map[i][j] == 'G' || map[i][j] == 'E') {
        unit *u = new unit();
        u->t = map[i][j] == 'G' ? GOBLIN : ELF;
        if (u->t == ELF) {
          u->power = elvPower;
        } else {
          u->power = 3;
        }
        u->pos = position(j, i);
        units.push_back(u);
        map[i][j] = '.';
      }
    }
  }
  return units;
}

inline void printUnits(list<unit *> units) {
  for (unit *u : units) {
    cout << (u->t == GOBLIN ? 'G' : 'E') << " pos: " << u->pos.x << ","
         << u->pos.y << " - " << u->health << endl;
  }
}

inline list<position> getPotentialTargets(list<unit *> units, unit *current) {
  list<position> targets;
  team searchFor = current->t == GOBLIN ? ELF : GOBLIN;
  for (unit *u : units) {
    if (u->t == searchFor && u->health > 0) {
      int x = u->pos.x, y = u->pos.y;
      targets.push_back(position(x, y - 1));
      targets.push_back(position(x, y + 1));
      targets.push_back(position(x - 1, y));
      targets.push_back(position(x + 1, y));
    }
  }
  return targets;
}

inline bool isUnitHere(position p, list<unit *> units) {
  for (unit *u : units) {
    if (u->pos == p && u->health > 0)
      return true;
  }
  return false;
}

struct queueNode {
  position myPos;
  queueNode *lastPos;
};

inline bool isValid(position p, size_t height, size_t width) {
  return p.x > -1 && p.y > -1 && p.x < width && p.y < height;
}

inline list<position> getShortestPath(position target, char **map,
                                      size_t height, size_t width,
                                      position current, list<unit *> units) {
  queue<queueNode *> q;
  int xNum[] = {0, -1, 1, 0};
  int yNum[] = {-1, 0, 0, 1};

  bool visited[height][width];
  memset(visited, false, sizeof visited);

  visited[current.y][current.x] = true;
  position invalid = position(-1, -1);
  q.push(new queueNode({current, new queueNode({invalid, nullptr})}));
  queueNode *curr = nullptr;
  bool isReachable = false;
  while (!q.empty() && !isReachable) {
    curr = q.front();

    if (curr->myPos == target) {
      isReachable = true;
      break;
    }
    q.pop();
    for (int i = 0; i < 4; i++) {
      position p = position(curr->myPos.x + xNum[i], curr->myPos.y + yNum[i]);
      if (map[p.y][p.x] == '.' && !visited[p.y][p.x] && !isUnitHere(p, units) &&
          isValid(p, height, width)) {
        visited[p.y][p.x] = true;
        queueNode *qu = new queueNode({p, curr});
        q.push(qu);
      }
    }
  }
  list<position> path;
  if (isReachable) {
    while (!(curr->myPos == invalid)) {
      path.push_front(curr->myPos);
      curr = curr->lastPos;
    }
  }

  return path;
}

// 82 2569
// 210658
inline list<list<position>> getShortestPaths(list<position> targets, char **map,
                                             size_t height, size_t width,
                                             position current,
                                             list<unit *> units) {
  list<list<position>> paths;
  size_t shortest = 1000000;
  for (position p : targets) {
    list<position> path =
        getShortestPath(p, map, height, width, current, units);
    if (path.size() > 0) {
      paths.push_back(path);
      if (path.size() < shortest)
        shortest = path.size();
    }
  }
  list<list<position>> ps;
  for (list<position> p : paths) {
    if (p.size() == shortest)
      ps.push_back(p);
  }
  return ps;
}

inline position getNextPosition(list<list<position>> paths) {
  list<position> pos;
  for (list<position> p : paths) {
    auto it = p.begin();
    pos.push_back(*(++it));
  }
  pos.sort();
  if (pos.size() > 0)
    return *pos.begin();
  else
    return position(-1, -1);
}

inline void printPosition(list<position> ps) {
  for (position p : ps)
    cout << p.x << "," << p.y << endl;
}

inline unit *getUnitAt(position p, list<unit *> units) {
  for (unit *u : units) {
    if (u->pos == p && u->health > 0)
      return u;
  }
  return nullptr;
}

inline void printMap(char **map, size_t height, size_t width,
                     list<unit *> units) {
  for (size_t i = 0; i < height; i++) {
    for (size_t j = 0; j < width; j++) {
      unit *u = getUnitAt(position(j, i), units);
      if (u) {
        cout << (u->t == GOBLIN ? 'G' : 'E');
      } else {
        cout << map[i][j];
      }
    }
    cout << endl;
  }
}

inline list<unit *> removeUnit(unit *toRemove, list<unit *> units) {
  for (auto it = units.begin(); it != units.end(); ++it) {
    if ((*it)->pos == toRemove->pos) {
      units.erase(it);
      break;
    }
  }
  return units;
}

inline bool uSort(const unit *u, const unit *u2) { return *u < *u2; }

inline bool attack(unit current, list<unit *> units) {
  list<unit *> posible;
  int minHealth = 200;
  bool willAttack = false;
  int xNum[] = {0, -1, 1, 0};
  int yNum[] = {-1, 0, 0, 1};
  for (int i = 0; i < 4; i++) {
    position p = position(current.pos.x + xNum[i], current.pos.y + yNum[i]);
    unit *u = getUnitAt(p, units);
    if (u && u->t != current.t) {
      willAttack = true;
      posible.push_back(u);
      if (u->health < minHealth)
        minHealth = u->health;
    }
  }
  if (willAttack) {
    list<unit *> attackable;
    for (unit *us : posible) {
      if (us->health == minHealth)
        attackable.push_back(us);
    }
    attackable.sort(uSort);
    unit *toAttack = *(attackable.begin());
    // cout << "Will attack " << (current.t == GOBLIN ? 'G' : 'E') << " "
    //      << current.pos.x << "," << current.pos.y << " -> "
    //      << (toAttack->t == GOBLIN ? 'G' : 'E') << " " << toAttack->pos.x
    //      << ", " << toAttack->pos.y << endl;
    toAttack->health -= current.power;
    return true;
  }
  // cout << "Won't attack" << endl;
  return false;
}

inline int getHealths(list<unit *> units) {
  int sum = 0;
  for (unit *u : units) {
    sum += u->health;
  }
  return sum;
}

inline bool runGame(int elvPower, bool stopOnElvDeath) {
  TxtReader reader;
  string file = "15/test.txt";
  vector<string> rows = reader.getStringFromFile(file);

  size_t height = rows.size();
  size_t width = rows[0].length();

  char **map = new char *[height];
  for (size_t i = 0; i < height; i++) {
    map[i] = new char[width];
    for (size_t j = 0; j < width; j++) {
      map[i][j] = rows[i][j];
    }
  }
  int gameRound = 0;
  list<unit *> units = getUnits(map, height, width, elvPower);
  bool moveMade = true;
  while (moveMade) {
    moveMade = false;
    units.sort(uSort);
    printMap(map, height, width, units);
    for (auto it = units.begin(); it != units.end(); ++it) {
      unit *current = *(it);
      if (current->health > 0) {
        if (!attack(*current, units)) {
          list<position> poTarg = getPotentialTargets(units, current);
          if (poTarg.size() < 1) {
            moveMade = false;
            gameRound -= 1;
            break;
          }
          // printPosition(poTarg);
          // cout << endl;
          list<list<position>> paths =
              getShortestPaths(poTarg, map, height, width, current->pos, units);
          // for (list<position> p : paths) {
          //   printPosition(p);
          //   cout << endl;
          // }
          if (paths.size() > 0) {
            moveMade = true;
            position next = getNextPosition(paths);
            if (next == position(-1, -1))
              ;
            else {
              current->pos = next;
              attack(*current, units);
            }
          }
        } else {
          moveMade = true;
        }
      }
    }
    bool dead = true;
    while (dead) {
      dead = false;
      unit *toRemove = nullptr;
      for (unit *u : units) {
        if (u->health < 1) {
          if (u->t == ELF && stopOnElvDeath) {
            printMap(map, height, width, units);
            return false;
          }
          toRemove = u;
          dead = true;
          break;
        }
      }
      if (dead) {
        units = removeUnit(toRemove, units);
      }
    }
    gameRound += 1;
  }
  printMap(map, height, width, units);
  printUnits(units);
  int healths = getHealths(units);
  cout << (gameRound) << " " << healths << endl;
  cout << (healths * (gameRound)) << endl;

  delete[] map;
  return ((*(units.begin()))->t == ELF);
}

int main() {
  // First task
  // runGame(3, false);

  // Second task
  bool elfLoose = true;
  int elvPower = 3;
  while (elfLoose) {
    elvPower += 1;
    cout << elvPower << endl;
    elfLoose = !runGame(elvPower, true);
  }
  cout << elvPower << endl;
  return 0;
}
