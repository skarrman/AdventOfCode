#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <numeric>

using namespace std;

class Exp
{
public:
  virtual long eval(long old)
  {
    return old;
  }
};

class AddExp : public Exp
{
private:
  Exp *lhs, *rhs;

public:
  AddExp(Exp *l, Exp *r)
  {
    lhs = l;
    rhs = r;
  }
  long eval(long old)
  {
    long l = lhs->eval(old);
    long r = rhs->eval(old);
    return l + r;
  }
};

class MultExp : public Exp
{
private:
  Exp *lhs, *rhs;

public:
  MultExp(Exp *l, Exp *r)
  {
    lhs = l;
    rhs = r;
  }
  long eval(long old)
  {
    long l = lhs->eval(old);
    long r = rhs->eval(old);
    return l * r;
  }
};

class ValExp : public Exp
{
private:
  long value;

public:
  ValExp(long v)
  {
    value = v;
  }
  long eval(long old)
  {
    return value;
  }
};

struct Test
{
  long div;
  size_t yay;
  size_t nay;
};

struct Monkey
{
  vector<long> items;
  Exp *operation;
  Test test;
};

Exp *parse_exp(string expStr)
{
  if (expStr.find("old") != string::npos)
    return new Exp();
  return new ValExp(stol(expStr));
}

vector<Monkey>
read_input()
{
  vector<Monkey> result;
  ifstream input_file("input.txt");
  string line;

  while (getline(input_file, line))
  {
    getline(input_file, line);
    line = line.substr(line.find(": ") + 2);
    vector<long> items;
    size_t to = line.find(",");
    while (to != string::npos)
    {
      items.push_back(stol(line.substr(0, to)));
      line = line.substr(to + 2);
      to = line.find(",");
    }
    items.push_back(stol(line));
    getline(input_file, line);
    line = line.substr(line.find("=") + 2);
    Exp *lhs = parse_exp(line.substr(0, line.find(" ")));
    Exp *rhs = parse_exp(line.substr(line.find_last_of(" ") + 1));
    Exp *op = line.substr(line.find(" ") + 1, 1) == "+" ? (Exp *)new AddExp(lhs, rhs) : (Exp *)new MultExp(lhs, rhs);
    getline(input_file, line);
    long test = stol(line.substr(line.find("by ") + 3));
    getline(input_file, line);
    size_t yay = stoul(line.substr(line.find_last_of(" ") + 1));
    getline(input_file, line);
    size_t nay = stoul(line.substr(line.find_last_of(" ") + 1));
    result.push_back({items, op, {test, yay, nay}});
    getline(input_file, line);
  }

  return result;
}

long solve(vector<Monkey> input, bool divByThree, size_t rounds)
{
  long d = input[0].test.div;
  for (auto m = ++input.begin(); m != input.end(); m++)
    d *= m->test.div;
  vector<size_t> inspected(input.size(), 0);
  for (int round = 0; round < rounds; round++)
  {
    for (int i = 0; i < input.size(); i++)
    {
      inspected[i] += input[i].items.size();
      for (auto level = input[i].items.begin(); level != input[i].items.end(); level++)
      {
        long newLevel = input[i].operation->eval(*level);
        if (divByThree)
          newLevel /= 3;
        else
          newLevel %= d;
        size_t to = newLevel % input[i].test.div == 0 ? input[i].test.yay : input[i].test.nay;
        input[to].items.push_back(newLevel);
      }
      input[i].items.clear();
    }
  }
  sort(inspected.begin(), inspected.end());
  return inspected[inspected.size() - 2] * inspected[inspected.size() - 1];
}

int main()
{
  vector<Monkey> input = read_input();

  char *part = getenv("part");
  if (part != NULL && string(part) == "part2")
    cout << solve(input, false, 10000) << endl;
  else
    cout << solve(input, true, 20) << endl;

  return 0;
}
