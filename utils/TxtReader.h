#ifndef TXTREADER_H
#define TXTREADER_H
#include <string>
#include <vector>

class TxtReader {
public:
  std::vector<int> getIntsFromFile(std::string);
  std::vector<std::string> getStringFromFile(std::string);
};

#endif // TXTREADER_H
