/*
 * Lunar Decompiler, a java decompiler.
 * Copyright (C) 2020 asger
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

#ifndef LDECOMP_TAG_TRAITS_HPP
#define LDECOMP_TAG_TRAITS_HPP

#include "constant_tag.hpp"

template <typename Type> struct tag_primitive_traits
{
  using data_class = ConstantData;

  static const constant_tag tag;
};
template <> struct tag_primitive_traits<std::string>
{
  using data_class = ConstantDataUtf8;

  static const constant_tag tag = constant_tag::Utf8;
};
template <> struct tag_primitive_traits<uint32_t>
{
  using data_class = ConstantDataNumeric<uint32_t>;

  static const constant_tag tag = constant_tag::Integer;
};
template <> struct tag_primitive_traits<uint64_t>
{
  using data_class = ConstantDataNumeric<uint64_t>;

  const constant_tag tag = constant_tag::Long;
};
template <> struct tag_primitive_traits<float>
{
  using data_class = ConstantDataNumeric<float>;

  static const constant_tag tag = constant_tag::Integer;
};
template <> struct tag_primitive_traits<double>
{
  using data_class = ConstantDataNumeric<double>;

  static const constant_tag tag = constant_tag::Double;
};

#endif // LDECOMP_TAG_TRAITS_HPP
