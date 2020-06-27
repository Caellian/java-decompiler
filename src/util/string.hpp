/*
 * Lunar Decompiler, a java decompiler.
 * Copyright (C) 2020 Tin Švagelj <tin.svagelj@live.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef LDECOMP_STRING_HPP
#define LDECOMP_STRING_HPP

#include <algorithm>
#include <istream>
#include <string>
#include <vector>

namespace util::string
{
template <typename StringType, typename DelimiterType>
std::vector<std::string> inline split_string(const StringType &str, const DelimiterType &delimiter)
{
  std::vector<std::string> res;

  size_t pos = 0;
  size_t at = 0;
  while (pos < str.size() && (at = str.find(delimiter, pos)) != std::string::npos)
  {
    if (pos != at)
    {
      res.push_back(str.substr(pos, at - pos));
    }
    pos = at + delimiter.size();
  }
  if (pos < str.size())
  {
    res.push_back(str.substr(pos, str.size() - pos));
  }

  return res;
}

template <typename StringType> std::vector<std::string> inline split_string(const StringType &str, char delimiter)
{
  std::vector<std::string> res;

  size_t pos = 0;
  size_t at = 0;
  while (pos < str.size() && (at = str.find(delimiter, pos)) != std::string::npos)
  {
    if (pos != at)
    {
      res.push_back(str.substr(pos, at - pos));
    }
    pos = at + 1;
  }
  if (pos < str.size())
  {
    res.push_back(str.substr(pos, str.size() - pos));
  }

  return res;
}

static inline std::string &ltrim(std::string &s)
{
  s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](int ch) {
            return !static_cast<bool>(std::isspace(ch));
          }));
  return s;
}

static inline std::string &rtrim(std::string &s)
{
  s.erase(std::find_if(s.rbegin(), s.rend(),
                       [](int ch) {
                         return !static_cast<bool>(std::isspace(ch));
                       })
              .base(),
          s.end());
  return s;
}

static inline std::string &trim(std::string &s)
{
  ltrim(s);
  rtrim(s);
  return s;
}

static inline std::string ltrim_copy(std::string s)
{
  ltrim(s);
  return s;
}

static inline std::string rtrim_copy(std::string s)
{
  rtrim(s);
  return s;
}

static inline std::string trim_copy(std::string s)
{
  trim(s);
  return s;
}

bool ends_with(const std::string &tested, const std::string &end);

bool starts_with(const std::string &tested, const std::string &beg);

template <typename StringType> std::istream &getline(std::istream &is, StringType &t)
{
  t.clear();

  std::istream::sentry se(is, true);
  std::streambuf *sb = is.rdbuf();

  while (true)
  {
    int c = sb->sbumpc();
    switch (c)
    {
    case '\n':
      return is;
    case '\r':
      if (sb->sgetc() == '\n')
      {
        sb->sbumpc();
      }
      return is;
    case std::streambuf::traits_type::eof():
      if (t.empty()) // line has no line ending
      {
        is.setstate(std::ios::eofbit);
      }
      return is;
    default:
      t += static_cast<typename StringType::value_type>(c);
    }
  }
}

} // namespace util::string

#endif // LDECOMP_STRING_HPP
