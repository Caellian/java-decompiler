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

#ifndef LDECOMP_ENDIAN_HPP
#define LDECOMP_ENDIAN_HPP

#include <climits>
#include <cstdint>
#include <cstdlib>

const uint8_t int8_width = 8;

namespace util::endian
{
template <typename Type> constexpr Type &reverse(Type &of)
{
  Type ref(of);
  size_t size = sizeof(Type);
  auto *data = reinterpret_cast<uint8_t *>(&of);

  for (size_t i = 0; i < size; ++i)
  {
    data[i] = static_cast<uint8_t>(ref >> ((size - i - 1) * int8_width)); // NOLINT(hicpp-signed-bitwise)
  }

  return of;
}

template <> constexpr float &reverse(float &of)
{
  return of;
}

template <> constexpr double &reverse(double &of)
{
  return of;
}

template <typename Type> constexpr Type reverse_copy(Type of)
{
  return reverse<Type>(of);
}

} // namespace util::endian

#endif // LDECOMP_ENDIAN_HPP
