#include "../utils/TxtReader.h"
#include <iostream>
#include <list>
#include <string>
#include <vector>

using namespace std;

struct Star {
  int x, y, vx, vy;
  void update();
};

void Star::update() {
  x += vx;
  y += vy;
}

inline list<Star *> getStars() {
  list<Star *> stars;
  TxtReader reader;
  vector<string> rows = reader.getStringFromFile("10/input.txt");

  string::size_type sz;
  for (string row : rows) {
    Star *star = new Star;
    string s = "";
    int i = 0;
    for (char c : row) {
      if (c != ',')
        s.push_back(c);
      else {
        switch (i) {
        case 0:
          star->x = stoi(s, &sz);
          s = "";
          i += 1;
          break;
        case 1:
          star->y = stoi(s, &sz);
          s = "";
          i += 1;
          break;
        case 2:
          star->vx = stoi(s, &sz);
          s = "";
          i += 1;
          break;
        case 3:
          star->vy = stoi(s, &sz);
          s = "";
          i += 0;
          break;
        }
      }
    }
    stars.push_back(star);
  }

  return stars;
}

inline list<Star *> updateStars(list<Star *> stars) {
  for (Star *s : stars)
    s->update();

  return stars;
}

inline void printSky(char **sky, int maxX, int maxY) {
  for (int i = 0; i < maxX; i++) {
    for (int j = 0; j < maxY; j++) {
      cout << sky[i][j] << " ";
    }
    cout << endl;
  }
}

int main() {
  list<Star *> stars = getStars();

  int seconds = 1;

  int maxXalignment = 0;
  int update = 0;

  while (seconds < 11000) {
    stars = updateStars(stars);
    int alingment = 0;
    int i = 0;
    for (auto it = stars.begin(); it != stars.end(); ++it) {
      i++;
      int j = 0;
      for (auto jt = stars.begin(); jt != stars.end(); ++jt) {
        j++;
        if (j <= i)
          continue;
        else {
          if ((*it)->x == (*jt)->x)
            alingment += 1;
          if ((*it)->y == (*jt)->y)
            alingment += 1;
        }
      }
    }
    if (alingment > maxXalignment) {
      maxXalignment = alingment;
      update = seconds;
    }
    seconds += 1;
  }

  list<Star *> newStars = getStars();
  for (int i = 0; i < update; i++)
    newStars = updateStars(newStars);
  int maxX = 0, maxY = 0, minX = 0, minY = 0;
  for (Star *s : newStars) {
    if (s->x > maxX)
      maxX = s->x;
    if (s->x < minX)
      minX = s->x;
    if (s->y > maxY)
      maxY = s->y;
    if (s->y < minY)
      minY = s->y;
  }

  int width = maxX - minX + 1;
  int height = maxY - minY + 1;

  for (Star *s : newStars) {
    s->x -= minX;
    s->y -= minY;
  }

  cout << width << " " << height << endl;

  char **sky = new char *[width];
  for (int i = 0; i < width; i++)
    sky[i] = new char[height];

  for (int i = 0; i < width; i++)
    for (int j = 0; j < height; j++)
      sky[i][j] = '_';

  for (Star *s : newStars) {
    sky[s->x][s->y] = '#';
  }
  printSky(sky, width, height);
  cout << update << " seconds" << endl;
}
