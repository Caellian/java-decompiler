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

#include "util/string.hpp"
#include <cinttypes>
#include <cstdio>
#include <spdlog/spdlog.h>
#include <algorithm>

inline bool can_open_file(const std::string &file)
{
#ifdef _MSC_VER
  FILE *tmp;
  return fopen_s(&tmp, file.c_str(), "r") != 0;
#else
  return std::fopen(file.c_str(), "re") != nullptr;
#endif
}

std::string read_text_file(unzFile file, const std::string &path) noexcept(false)
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
    int32_t read_size;
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

BinaryObjectBuffer *read_bin_file(unzFile file, const std::string &path) noexcept(false)
{
  if (unzLocateFile(file, path.c_str(), 0) == UNZ_END_OF_LIST_OF_FILE)
  {
    throw jar_file_error("jar entry not found", path);
  }
  uint64_t size;
  {
    unz_file_info fi;
    unzGetCurrentFileInfo(file, &fi, nullptr, 0, nullptr, 0, nullptr, 0);
    size = fi.uncompressed_size;
  }
  if (size == 0)
  {
    // File is empty
    return nullptr;
  }

  unzOpenCurrentFile(file);
  auto *result = new BinaryObjectBuffer(size);
  if (size > UINT32_MAX)
  {
    uint64_t pos = 0;
    int32_t read_size = 0;
    while ((read_size = unzReadCurrentFile(file, result->data() + pos, UINT32_MAX)) > 0)
    {
      pos += static_cast<uint64_t>(read_size);
    }
  }
  else
  {
    unzReadCurrentFile(file, result->data(), static_cast<uint32_t>(size));
  }
  unzCloseCurrentFile(file);

  return result;
}

JarFile::JarFile(const std::string &path)
    : m_absolute_path(std::filesystem::absolute(std::filesystem::path(path)).string())
{
  if (!can_open_file(m_absolute_path))
  {
    throw file_inaccessible_error(m_absolute_path);
  }

  auto *f = unzOpen(m_absolute_path.c_str());
  unzClose(f);
}

std::vector<std::string> JarFile::files() const
{
  if (m_absolute_path.empty())
  {
    throw jar_state_error("jar path not specified");
  }

  unzFile file = unzOpen(m_absolute_path.c_str());
  unz_global_info info;
  unzGetGlobalInfo(file, &info);

  if (file == nullptr)
  {
    throw file_inaccessible_error("file inaccessible or not a zip archive", m_absolute_path);
  }

  std::vector<std::string> res;
  res.reserve(info.number_entry);

  unzGoToFirstFile(file);
  do
  {
    unz_file_info file_info;
    unzGetCurrentFileInfo(file, &file_info, nullptr, 0, nullptr, 0, nullptr, 0);

    std::string name;
    name.resize(file_info.size_filename);

#pragma warning(disable : 4267)
    unzGetCurrentFileInfo(file, nullptr, name.data(), name.size() + 1, nullptr, 0, nullptr, 0);
#pragma warning(default : 4267)

    res.push_back(name);
  } while (unzGoToNextFile(file) != UNZ_END_OF_LIST_OF_FILE);
  unzClose(file);

  return res;
}

std::optional<std::istringstream> JarFile::openTextFile(const std::string &jar_path) noexcept
{
  auto *f = unzOpen(m_absolute_path.c_str());
  try
  {
    auto res = read_text_file(f, jar_path);
    unzClose(f);
    return std::optional<std::istringstream> {std::istringstream(res, std::ios::in)};
  }
  catch (const jar_file_error &)
  {
    unzClose(f);
    return std::nullopt;
  }
}

BinaryObjectBuffer *JarFile::openBinaryFile(const std::string &jar_path) noexcept
{
  auto *f = unzOpen(m_absolute_path.c_str());
  try
  {
    auto *result = read_bin_file(f, jar_path);
    unzClose(f);
    return result;
  }
  catch (const jar_file_error &)
  {
    unzClose(f);
    return nullptr;
  }
}

SectionedPairs JarFile::manifest()
{
  auto manifest = SectionedPairs {};

  auto mfso = openTextFile("META-INF/MANIFEST.MF");

  if (!mfso.has_value()) {
    return manifest;
  }

  auto &mfs = mfso.value(); // mfso is in function scope so reference won't be invalidated until return

  std::string line;

  std::string section_name = manifest_main_section;
  std::map<std::string, std::string> section;
  std::string last_prop;
  while (util::string::getline(mfs, line))
  {
    if (line.empty()) {
      if (!section.empty()) {
        manifest[section_name] = std::move(section);

        section_name = "";
        section = std::map<std::string, std::string>();
        last_prop = "";
      }
      continue;
    }

    if (std::find(line.begin(), line.end(), ':') != line.end()) {
      auto tokens = util::string::split_string(line, ':');
      last_prop = util::string::trim(tokens[0]);

      if (last_prop == "Name") {
        section_name = tokens[1].substr(1);
      } else {
        section[last_prop] = tokens[1].substr(1);
      }
    } else if (util::string::starts_with(line, " ")) { // assume continuation
      if (last_prop == "Name") {
        section_name += util::string::trim(line);
      } else {
        section[last_prop] += util::string::trim(line);
      }
    } else {
      throw std::runtime_error("unable to parse manifest");
    }
  }

  return manifest;
}
