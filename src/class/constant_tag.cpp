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

#include "constant_tag.hpp"

std::string constant_tag_name(constant_tag of)
{
  switch (of)
  {
  case constant_tag::Utf8:
    return "Utf8";
  case constant_tag::Integer:
    return "Integer";
  case constant_tag::Float:
    return "Float";
  case constant_tag::Long:
    return "Long";
  case constant_tag::Double:
    return "Double";
  case constant_tag::Class:
    return "Class";
  case constant_tag::String:
    return "String";
  case constant_tag::FieldReference:
    return "FieldReference";
  case constant_tag::MethodReference:
    return "MethodReference";
  case constant_tag::InterfaceMethodReference:
    return "InterfaceMethodReference";
  case constant_tag::NameAndType:
    return "NameAndType";
  case constant_tag::MethodHandle:
    return "MethodHandle";
  case constant_tag::MethodType:
    return "MethodType";
  case constant_tag::Dynamic:
    return "Dynamic";
  case constant_tag::InvokeDynamic:
    return "InvokeDynamic";
  case constant_tag::Module:
    return "Module";
  case constant_tag::Package:
    return "Package";
  default:
    return "";
  }
}
uint16_t constant_tag_offset(constant_tag of)
{
  switch (of)
  {
  case constant_tag::Long:
  case constant_tag::Double:
    return 2;
  default:
    return 1;
  }
}