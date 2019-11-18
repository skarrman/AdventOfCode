#include "date.h"
#include <iostream>
#include <string>

using namespace std;

Date::Date(string dateString) {
  year = parseYear(dateString);
  month = parseMonth(dateString);
  day = parseDay(dateString);
  hour = parseHour(dateString);
  minute = parseMinute(dateString);
}

void Date::setDate(string dateString) {
  year = parseYear(dateString);
  month = parseMonth(dateString);
  day = parseDay(dateString);
  hour = parseHour(dateString);
  minute = parseMinute(dateString);
}

Date::Date() {}

bool Date::operator<(const Date rhs) {
  if (year < rhs.year) {
    return true;
  } else if (year > rhs.year) {
    return false;
  } else {
    if (month < rhs.month) {
      return true;
    } else if (month > rhs.month) {
      return false;
    } else {
      if (day < rhs.day) {
        return true;
      } else if (day > rhs.day) {
        return false;
      } else {
        if (hour < rhs.hour) {
          return true;
        } else if (hour > rhs.hour) {
          return false;
        } else {
          if (minute < rhs.minute) {
            return true;
          } else if (minute > rhs.minute) {
            return false;
          }
        }
      }
    }
  }
  return false;
}

//[1518-11-05 00:55] wakes up
int Date::parseYear(string dateString) {
  string::size_type sz;
  size_t start = dateString.find_first_of(dateString[0]);
  size_t end = dateString.find_first_of('-');

  return stoi(dateString.substr(start, end), &sz);
}
int Date::parseMonth(string dateString) {
  string::size_type sz;
  size_t start = dateString.find_first_of('-');
  size_t end = dateString.find_last_of('-');

  return stoi(dateString.substr(start + 1, end), &sz);
}
int Date::parseDay(string dateString) {
  string::size_type sz;
  size_t start = dateString.find_last_of('-');
  size_t end = dateString.find_first_of(' ');

  return stoi(dateString.substr(start + 1, end), &sz);
}
int Date::parseHour(string dateString) {
  string::size_type sz;
  size_t start = dateString.find_first_of(' ');
  size_t end = dateString.find_first_of(':');

  return stoi(dateString.substr(start + 1, end), &sz);
}
int Date::parseMinute(string dateString) {
  string::size_type sz;
  size_t start = dateString.find_first_of(':');
  size_t end = dateString.size() - 1;

  return stoi(dateString.substr(start + 1, end), &sz);
}

string Date::toString() {
  return to_string(year) + "-" + to_string(month) + "-" + to_string(day) + " " +
         to_string(hour) + ":" + to_string(minute);
}
