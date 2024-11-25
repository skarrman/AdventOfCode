#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

vector<vector<vector<char> *> *> *read_input()
{
  vector<vector<vector<char> *> *> *result = new vector<vector<vector<char> *> *>();
  vector<vector<char> *> *map = new vector<vector<char> *>();
  vector<char> *row = new vector<char>();
  ifstream input_file("input.txt");
  string line;

  char ch, last;
  fstream fin("input.txt", fstream::in);
  while (fin >> noskipws >> ch)
  {
    if (ch == '\n' && last == '\n')
    {
      result->push_back(map);
      map = new vector<vector<char> *>();
    }
    else if (ch == '\n')
    {
      map->push_back(row);
      row = new vector<char>();
    }
    else
      row->push_back(ch);
    last = ch;
  }
  map->push_back(row);
  result->push_back(map);

  return result;
}

void print(vector<vector<char> *> *map)
{
  int rows = 0, cols = 0;
  for (auto row = map->begin(); row != map->end(); ++row)
  {
    rows++;
    int c = 0;
    for (auto col = (*row)->begin(); col != (*row)->end(); ++col)
    {
      c++;
    }
    cols = c;
  }
}

void print(vector<vector<vector<char> *> *> *input)
{
  for (auto map = input->begin(); map != input->end(); ++map)
  {
    print(*map);
  }
}

bool isRowReflection(vector<char> *row, int x)
{
  for (int i = x, j = x + 1; i >= 0 && j < row->size(); i--, j++)
  {
    if ((*row)[i] != (*row)[j])
      return false;
  }
  return true;
}

bool validateRows(vector<vector<char> *> *map, int x)
{
  for (auto row = map->begin(); row != map->end(); ++row)
  {
    if (!isRowReflection(*row, x))
      return false;
  }
  return true;
}

bool isColReflection(vector<vector<char> *> *map, int col, int x)
{
  for (int i = x, j = x + 1; i >= 0 && j < map->size(); i--, j++)
  {
    if ((*(*map)[i])[col] != (*(*map)[j])[col])
      return false;
  }
  return true;
}

bool validateCol(vector<vector<char> *> *map, int x)
{
  for (int i = 0; i < (*map)[0]->size(); i++)
  {
    if (!isColReflection(map, i, x))
      return false;
  }
  return true;
}

void solve(vector<vector<char> *> *map, int *cols, int *rows)
{
  for (auto x = 0; x < (*map)[0]->size() - 1; x++)
  {
    if (isRowReflection((*map)[0], x) && validateRows(map, x))
    {
      *cols = x + 1;
      return;
    }
  }

  for (auto x = 0; x < map->size() - 1; x++)
  {
    if (isColReflection(map, 0, x) && validateCol(map, x))
    {

      *rows = x + 1;
      return;
    }
  }
  throw -1;
}

int solve1(vector<vector<vector<char> *> *> *input)
{
  int colsToLeft = 0;
  int rowsAbove = 0;
  int mapn = 0;
  for (auto map = input->begin(); map != input->end(); ++map)
  {
    int cols = 0, rows = 0;
    solve(*map, &cols, &rows);
    colsToLeft += cols;
    rowsAbove += rows;
  }

  return colsToLeft + 100 * rowsAbove;
}

int solve2(vector<vector<vector<char> *> *> *input)
{
  int colsToLeft = 0;
  int rowsAbove = 0;
  int mapn = 0;
  for (auto map = input->begin(); map != input->end(); ++map)
  {
    int cols = -1, rows = -1;
    solve(*map, &cols, &rows);
    bool reflectionFound = false;
    for (int i = 0; i < (*map)->size(); i++)
    {
      for (int j = 0; j < (**map)[0]->size(); j++)
      {
        char prev = (*(**map)[i])[j];
        (*(**map)[i])[j] = prev == '.' ? '#' : '.';

        for (auto x = 0; x < (**map)[0]->size() - 1; x++)
        {
          if ((x + 1) != cols && isRowReflection((**map)[i], x) && validateRows(*map, x))
          {
            colsToLeft += x + 1;
            reflectionFound = true;
            break;
          }
        }
        if (reflectionFound)
          break;
        for (auto x = 0; x < (*map)->size() - 1; x++)
        {
          if ((x + 1) != rows && isColReflection(*map, j, x) && validateCol(*map, x))
          {
            reflectionFound = true;
            rowsAbove += x + 1;
            break;
          }
        }
        (*(**map)[i])[j] = prev;
        if (reflectionFound)
          break;
      }
      if (reflectionFound)
        break;
    }
  }

  return colsToLeft + 100 * rowsAbove;
}

int main()
{
  vector<vector<vector<char> *> *> *input = read_input();
  print(input);

  char *part = getenv("part");
  if (part == NULL)
  {
    printf("%d\n", solve1(input));
  }
  else if (string(part) == "part1")
  {
    printf("%d\n", solve1(input));
  }
  else if (string(part) == "part2")
  {
    printf("%d\n", solve2(input));
  }
  else
  {
  }

  return 0;
}
