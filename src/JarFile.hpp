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

#ifndef LDECOMP_JARFILE_HPP
#define LDECOMP_JARFILE_HPP

#include "error/jar_error.hpp"
#include "util/BinaryObjectBuffer.hpp"

#include <filesystem>
#include <map>
#include <minizip/unzip.h>
#include <optional>
#include <ostream>
#include <utility>
#include <vector>
#include <mutex>

const std::string manifest_main_section = "@[main_section]";
using SectionedPairs = std::map<std::string, std::map<std::string, std::string>>;

class JarFile
{
  std::string m_absolute_path;

public:
  class iterator {
  public:
    using iterator_category = std::forward_iterator_tag;
    using value_type = BinaryObjectBuffer;
    using difference_type = void;
    using pointer = BinaryObjectBuffer*;
    using reference = BinaryObjectBuffer&;

  };

  explicit JarFile(const std::string &path) noexcept(false);

  [[nodiscard]] const std::string &path() const
  {
    return m_absolute_path;
  };

  [[nodiscard]] SectionedPairs manifest();

  [[nodiscard]] std::optional<std::istringstream> openTextFile(const std::string &jar_path) noexcept;
  [[nodiscard]] BinaryObjectBuffer *openBinaryFile(const std::string &jar_path) noexcept;

  [[nodiscard]] std::vector<std::string> files()
      const; // TODO: Return an iterator. This consumes a lot of memory with large archives.
};

#endif // LDECOMP_JARFILE_HPP