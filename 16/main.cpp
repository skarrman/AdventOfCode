#include "../utils/TxtReader.h"
#include <iostream>
#include <map>
#include <vector>
using namespace std;
#define A 0
#define B 1
#define C 2
#define REGSIZE 4

typedef int *(*opr)(int *, int *);

struct testCase {
  int *before, *after, *instruction, num;
};

inline int *addr(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] + reg[instruction[B]];
  return reg;
}

inline int *addi(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] + instruction[B];
  return reg;
}

inline int *mulr(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] * reg[instruction[B]];
  return reg;
}

inline int *muli(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] * instruction[B];
  return reg;
}

inline int *banr(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] & reg[instruction[B]];
  return reg;
}

inline int *bani(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] & instruction[B];
  return reg;
}

inline int *borr(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] | reg[instruction[B]];
  return reg;
}

inline int *bori(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] | instruction[B];
  return reg;
}

inline int *setr(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]];
  return reg;
}

inline int *seti(int *reg, int *instruction) {
  reg[instruction[C]] = instruction[A];
  return reg;
}

inline int *gtir(int *reg, int *instruction) {
  reg[instruction[C]] = instruction[A] > reg[instruction[B]] ? 1 : 0;
  return reg;
}

inline int *gtri(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] > instruction[B] ? 1 : 0;
  return reg;
}

inline int *gtrr(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] > reg[instruction[B]] ? 1 : 0;
  return reg;
}

inline int *eqir(int *reg, int *instruction) {
  reg[instruction[C]] = instruction[A] == reg[instruction[B]] ? 1 : 0;
  return reg;
}

inline int *eqri(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] == instruction[B] ? 1 : 0;
  return reg;
}

inline int *eqrr(int *reg, int *instruction) {
  reg[instruction[C]] = reg[instruction[A]] == reg[instruction[B]] ? 1 : 0;
  return reg;
}

inline bool isEqual(int *reg, int *expected) {
  for (int i = 0; i < REGSIZE; i++) {
    if (reg[i] != expected[i])
      return false;
  }
  return true;
}

inline int *getCopy(int *original) {
  int *before = new int[REGSIZE];
  for (int i = 0; i < REGSIZE; i++)
    before[i] = original[i];
  return before;
}

inline int testInstruction(int *original, int *instruction, int *after, int num,
                           vector<opr> operations) {
  int match = 0;
  for (opr o : operations) {
    int *before = getCopy(original);
    if (isEqual(o(before, instruction), after)) {
      match += 1;
    }
  }
  return match;
}

inline vector<opr> getOperations() {
  vector<opr> operations;
  operations.push_back(&addr);
  operations.push_back(&addi);
  operations.push_back(&mulr);
  operations.push_back(&muli);
  operations.push_back(&banr);
  operations.push_back(&bani);
  operations.push_back(&borr);
  operations.push_back(&bori);
  operations.push_back(&setr);
  operations.push_back(&seti);
  operations.push_back(&gtir);
  operations.push_back(&gtri);
  operations.push_back(&gtrr);
  operations.push_back(&eqir);
  operations.push_back(&eqri);
  operations.push_back(&eqrr);
  return operations;
}

inline map<int, opr> mapInstructions(vector<testCase> testCases) {
  vector<opr> operations = getOperations();
  map<int, opr> opMap;
  while (operations.size() > 0) {
    for (testCase tc : testCases) {
      int match = 0;
      size_t i = 0;
      size_t index = 0;
      for (opr o : operations) {
        int *before = getCopy(tc.before);
        if (isEqual(o(before, tc.instruction), tc.after)) {
          match += 1;
          index = i;
        }
        i++;
      }
      if (match == 1) {
        opMap.insert({tc.num, operations[index]});
        operations.erase(operations.begin() + index);
      }
    }
    cout << operations.size() << " " << opMap.size() << endl;
  }
  return opMap;
}

// Before:[3,2,1,1]
// 9212
// After:[3,2,2,1]
inline vector<testCase> getTestCases(vector<string> rows) {
  vector<testCase> testCases;

  for (auto it = rows.begin(); it != rows.end(); ++it) {
    it++;
    int *before = new int[REGSIZE];
    before[0] = (*it)[8] - '0';
    before[1] = (*it)[10] - '0';
    before[2] = (*it)[12] - '0';
    before[3] = (*it)[14] - '0';
    it++;
    int *instruction = new int[REGSIZE - 1];
    int num;
    if ((*it).size() == 4) {
      instruction[0] = (*it)[1] - '0';
      instruction[1] = (*it)[2] - '0';
      instruction[2] = (*it)[3] - '0';
      num = (*it)[0] - '0';
    } else {
      string::size_type sz;
      instruction[0] = (*it)[2] - '0';
      instruction[1] = (*it)[3] - '0';
      instruction[2] = (*it)[4] - '0';
      num = stoi((*it).substr(0, 2), &sz);
    }
    it++;
    int *after = new int[REGSIZE];
    after[0] = (*it)[7] - '0';
    after[1] = (*it)[9] - '0';
    after[2] = (*it)[11] - '0';
    after[3] = (*it)[13] - '0';
    testCases.push_back({before, after, instruction, num});
  }
  return testCases;
}

struct inst {
  int num, *instruction;
};

inline vector<inst> getTestInstructions(vector<string> rows) {
  vector<inst> insts;
  for (auto it = rows.begin(); it != rows.end(); ++it) {
    int *instruction = new int[REGSIZE - 1];
    int num;
    if ((*it).size() == 4) {
      instruction[0] = (*it)[1] - '0';
      instruction[1] = (*it)[2] - '0';
      instruction[2] = (*it)[3] - '0';
      num = (*it)[0] - '0';
    } else {
      string::size_type sz;
      instruction[0] = (*it)[2] - '0';
      instruction[1] = (*it)[3] - '0';
      instruction[2] = (*it)[4] - '0';
      num = stoi((*it).substr(0, 2), &sz);
    }
    insts.push_back({num, instruction});
  }

  return insts;
}

inline void printCases(vector<testCase> testCases) {
  for (testCase tc : testCases) {
    for (int i = 0; i < REGSIZE; i++) {
      cout << tc.before[i] << " ";
    }
    cout << endl << tc.num << endl;
    for (int i = 0; i < REGSIZE - 1; i++) {
      cout << tc.instruction[i] << " ";
    }
    cout << endl;
    for (int i = 0; i < REGSIZE; i++) {
      cout << tc.after[i] << " ";
    }
    cout << endl;
  }
}

inline int *doTest(vector<inst> instructions, map<int, opr> opMap) {
  int *reg = new int[REGSIZE];
  memset(reg, 0, sizeof reg);

  for (inst in : instructions) {
    reg = opMap[in.num](reg, in.instruction);
  }
  return reg;
}

int main() {
  TxtReader reader;
  vector<string> rows = reader.getStringFromFile("16/input.txt");
  vector<testCase> testCases = getTestCases(rows);
  //   printCases(testCases);
  vector<opr> operations = getOperations();
  int sum = 0;
  for (testCase tc : testCases) {
    sum += testInstruction(tc.before, tc.instruction, tc.after, tc.num,
                           operations) > 2
               ? 1
               : 0;
  }
  cout << sum << endl;
  // Second task
  rows = reader.getStringFromFile("16/test.txt");
  vector<inst> insts = getTestInstructions(rows);
  map<int, opr> opMap = mapInstructions(testCases);
  int *reg = doTest(insts, opMap);
  cout << reg[0] << endl;
  return 0;
}
