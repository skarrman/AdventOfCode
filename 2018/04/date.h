#ifndef DATE_H
#define DATE_H
#include <string>

class Date {
public:
  int year, month, day, hour, minute;
  Date(std::string);
  Date();
  void setDate(std::string);
  int compare(Date);
  std::string toString();
  bool operator<(const Date);

private:
  int parseYear(std::string);
  int parseMonth(std::string);
  int parseDay(std::string);
  int parseHour(std::string);
  int parseMinute(std::string);
};

#endif // DATE_H
