#include "../utils/TxtReader.h"
#include <iostream>
#include <map>
#include <string>
#include <vector>
using namespace std;

struct Node {
  vector<Node *> children;
  vector<int> meta;
};

// Setting up tree in correct structure

inline int *getTree(int *lentght) {
  TxtReader reader;
  string row = (reader.getStringFromFile("08/input.txt"))[0];
  vector<int> tree;
  string::size_type sz;

  auto it = row.begin();

  while (it != row.end()) {
    string s = "";
    while (*it != ' ') {
      s.push_back(*it);
      it++;
      if (it == row.end()) {
        it--;
        break;
      }
    }
    tree.push_back(stoi(s, &sz));
    it++;
  }

  tree.shrink_to_fit();
  *lentght = tree.size();
  int *intTree = new int[tree.size()];
  int i = 0;
  for (int n : tree) {
    intTree[i] = n;
    i += 1;
  }
  return intTree;
}

inline Node *setUpTree(int *tree, int *nodeStart, int size) {
  if (*nodeStart >= size) {
    return nullptr;
  } else if (tree[*nodeStart] == 0) {
    Node *node = new Node;

    for (int i = *nodeStart + 2; i < *nodeStart + 2 + tree[*nodeStart + 1]; i++)
      node->meta.push_back(tree[i]);
    *nodeStart = *nodeStart + 2 + tree[*nodeStart + 1];
    return node;
  } else {
    int meta = tree[*nodeStart + 1];
    int children = tree[*nodeStart];
    *nodeStart += 2;
    Node *node = new Node;

    for (int i = 0; i < children; i++) {
      node->children.push_back(setUpTree(tree, nodeStart, size));
    }
    for (int i = *nodeStart; i < *nodeStart + meta; i++) {
      node->meta.push_back(tree[i]);
    }
    *nodeStart += meta;
    return node;
  }
}

// The actual tasks

inline int getSum(Node *node) {
  int sum = 0;
  for (auto it = node->meta.begin(); it != node->meta.end(); ++it) {
    sum += *it;
  }
  for (auto it = node->children.begin(); it != node->children.end(); ++it) {
    sum += getSum(*it);
  }
  return sum;
}

inline int getSecondSum(Node *node) {
  int sum = 0;
  if (node->children.size() == 0) {
    for (auto it = node->meta.begin(); it != node->meta.end(); ++it)
      sum += *it;
  }
  for (auto it = node->meta.begin(); it != node->meta.end(); ++it) {
    if (*it - 1 < node->children.size()) {
      sum += getSecondSum(node->children[*it - 1]);
    }
  }
  return sum;
}

int main() {
  int *size = new int;
  int *tree = getTree(size);
  int *nodeStart = new int;
  *nodeStart = 0;
  Node *root = setUpTree(tree, nodeStart, *size);

  cout << getSum(root) << endl;
  cout << getSecondSum(root) << endl;

  delete[] tree;
  delete size;
  delete nodeStart;
  delete root;
  return 0;
}
