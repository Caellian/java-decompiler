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

#ifndef LDECOMP_CLASS_FORMAT_ERROR_HPP
#define LDECOMP_CLASS_FORMAT_ERROR_HPP

#include <bit>
#include <stdexcept>

struct class_format_error : public std::runtime_error
{
  explicit class_format_error(const std::string &desc) noexcept : runtime_error(desc) {}
  class_format_error(class_format_error &&other) noexcept : runtime_error(other.what()) {}
  class_format_error(const class_format_error &other) noexcept : runtime_error(other.what()) {}
  ~class_format_error() noexcept override = default;

  class_format_error &operator=(const class_format_error &other) = default;
  class_format_error &operator=(class_format_error &&other) = default;
};

#endif // LDECOMP_CLASS_FORMAT_ERROR_HPP
