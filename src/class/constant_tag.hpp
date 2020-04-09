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

#ifndef LDECOMP_CONSTANT_TAG_HPP
#define LDECOMP_CONSTANT_TAG_HPP

#include <string>

enum class constant_tag
{
  Utf8 = 1,
  // 2?
  Integer = 3,
  Float = 4,
  Long = 5,
  Double = 6,
  Class = 7,
  String = 8,
  FieldReference = 9,
  MethodReference = 10,
  InterfaceMethodReference = 11,
  NameAndType = 12,
  // 13?
  // 14?
  MethodHandle = 15,
  MethodType = 16,
  Dynamic = 17,
  InvokeDynamic = 18,
  Module = 19,
  Package = 20
};

std::string constant_tag_name(constant_tag of);

uint16_t constant_tag_offset(constant_tag of);

#endif // LDECOMP_CONSTANT_TAG_HPP
