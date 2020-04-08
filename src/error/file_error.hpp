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

#ifndef LDECOMP_FILE_ERROR_HPP
#define LDECOMP_FILE_ERROR_HPP

#include <stdexcept>
#include <utility>

class file_error : public std::runtime_error
{
  std::string m_file;

public:
  file_error(const std::string &desc, const std::string &file) noexcept // NOLINT(modernize-pass-by-value)
      : runtime_error(desc), m_file(file)
  {
  }
  explicit file_error(const std::string &file) noexcept // NOLINT(modernize-pass-by-value)
      : runtime_error("generic file error"), m_file(file)
  {
  }
  file_error(file_error &&other) noexcept : runtime_error(other.what()), m_file(std::move(other.m_file)) {}
  file_error(const file_error &other) noexcept : runtime_error(other.what()), m_file(other.m_file) {}
  ~file_error() noexcept override = default;

  file_error &operator=(const file_error &other) = default;
  file_error &operator=(file_error &&other) = default;

  [[nodiscard]] std::string getFile() const noexcept
  {
    return m_file;
  };
};

struct file_inaccessible_error : public file_error
{
  file_inaccessible_error(const std::string &desc, const std::string &file) noexcept : file_error(desc, file) {}
  explicit file_inaccessible_error(const std::string &file) noexcept : file_error("file is inaccessible", file) {}
  file_inaccessible_error(file_inaccessible_error &&other) noexcept : file_error(other.what(), other.getFile()) {}
  file_inaccessible_error(const file_inaccessible_error &other) noexcept : file_error(other.what(), other.getFile()) {}
  ~file_inaccessible_error() noexcept override = default;

  file_inaccessible_error &operator=(const file_inaccessible_error &other) = default;
  file_inaccessible_error &operator=(file_inaccessible_error &&other) = default;
};

#endif // LDECOMP_FILE_ERROR_HPP
