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

#ifndef LDECOMP_JAR_ERROR_HPP
#define LDECOMP_JAR_ERROR_HPP

#include "file_error.hpp"

struct jar_state_error : public std::logic_error
{
  explicit jar_state_error(const std::string &desc) noexcept : logic_error(desc) {}
  jar_state_error(jar_state_error &&other) noexcept : logic_error(other.what()) {}
  jar_state_error(const jar_state_error &other) noexcept : logic_error(other.what()) {}
  ~jar_state_error() noexcept override = default;

  jar_state_error &operator=(const jar_state_error &other) = default;
  jar_state_error &operator=(jar_state_error &&other) = default;
};

struct jar_file_error : public file_error
{
  explicit jar_file_error(const std::string &desc, const std::string &file) noexcept : file_error(desc, file) {}
  jar_file_error(jar_file_error &&other) noexcept : file_error(other.what(), other.getFile()) {}
  jar_file_error(const jar_file_error &other) noexcept : file_error(other.what(), other.getFile()) {}
  ~jar_file_error() noexcept override = default;

  jar_file_error &operator=(const jar_file_error &other) = default;
  jar_file_error &operator=(jar_file_error &&other) = default;
};

#endif // LDECOMP_JAR_ERROR_HPP
