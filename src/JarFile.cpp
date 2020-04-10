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

#include "JarFile.hpp"

#include <cinttypes>
#include <cstdio>
#include <spdlog/spdlog.h>

inline bool can_open_file(const std::string &file)
{
#ifdef _MSC_VER
  FILE *tmp;
  std::wstring w_clone(file.length(), L' ');
  std::copy(file.begin(), file.end(), w_clone.begin());
  return fopen_s(&tmp, w_clone.c_str(), L"r") != 0;
#else
  return std::fopen(file.c_str(), "re") != nullptr;
#endif
}

JarFile::JarFile(const std::string &path)
    : absolute_path(std::filesystem::absolute(std::filesystem::path(path)).string())
{
  if (!can_open_file(absolute_path))
  {
    throw file_inaccessible_error(absolute_path);
  }
}

std::vector<std::string> JarFile::files() const
{
  if (absolute_path.empty())
  {
    throw jar_state_error("jar path not specified");
  }

  std::vector<std::string> res;

  unzFile file = unzOpen(absolute_path.c_str());
  unz_global_info info;
  unzGetGlobalInfo(file, &info);
  res.reserve(info.number_entry);

  if (file == nullptr)
  {
    throw file_inaccessible_error("file inaccessible or not a zip archive", absolute_path);
  }

  unzGoToFirstFile(file);
  do
  {
    unz_file_info file_info;
    unzGetCurrentFileInfo(file, &file_info, nullptr, 0, nullptr, 0, nullptr, 0);

    std::string name;
    name.resize(file_info.size_filename);
    unzGetCurrentFileInfo(file, nullptr, name.data(), name.size() + 1, nullptr, 0, nullptr, 0);
    res.push_back(name);
  } while (unzGoToNextFile(file) != UNZ_END_OF_LIST_OF_FILE);
  unzClose(file);

  return res;
}

JarFile &JarFile::open(const std::string &path)
{
  absolute_path = std::filesystem::absolute(std::filesystem::path(path)).string();
  if (!can_open_file(path))
  {
    throw file_inaccessible_error(absolute_path);
  }
  return *this;
}

std::string read_jar_file_contents(unzFile file, const std::string &path)
{
  if (unzLocateFile(file, path.c_str(), 0) == UNZ_END_OF_LIST_OF_FILE)
  {
    throw jar_file_error("jar entry not found", path);
  }
  uint64_t size = 0;
  {
    unz_file_info fi;
    unzGetCurrentFileInfo(file, &fi, nullptr, 0, nullptr, 0, nullptr, 0);
    size = fi.uncompressed_size;
  }
  if (size == 0)
  { // File is empty
    return "";
  }

  unzOpenCurrentFile(file);
  std::string result;
  result.resize(size);
  if (size > UINT32_MAX)
  {
    uint64_t pos = 0;
    int32_t read_size = 0;
    while ((read_size = unzReadCurrentFile(file, result.data() + pos, UINT32_MAX)) > 0)
    {
      pos += static_cast<uint64_t>(read_size);
    }
  }
  else
  {
    unzReadCurrentFile(file, result.data(), static_cast<uint32_t>(size));
  }
  unzCloseCurrentFile(file);

  return result;
}

std::optional<std::istringstream> JarFile::openTextFile(const std::string &jar_path) noexcept
{
  auto f = unzOpen(absolute_path.c_str());
  try
  {
    auto res = read_jar_file_contents(f, jar_path);
    unzClose(f);
    return std::optional<std::istringstream> {std::istringstream(res, std::ios::in)};
  }
  catch (const jar_file_error &err)
  {
    unzClose(f);
    return std::nullopt;
  }
}

std::optional<util::IObjStream> JarFile::openBinaryFile(const std::string &jar_path) noexcept
{
  auto f = unzOpen(absolute_path.c_str());
  try
  {
    auto res = read_jar_file_contents(f, jar_path);
    unzClose(f);
    return std::optional<util::IObjStream> {util::IObjStream(res, std::ios::in | std::ios::binary)};
  }
  catch (const jar_file_error &err)
  {
    unzClose(f);
    return std::nullopt;
  }
}
