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

#include "MemberInfo.hpp"

#include "../error/class_format_error.hpp"
#include "ConstantData.hpp"
#include <utility>

MemberInfo::MemberInfo(std::bitset<field_access_size> access_flags, std::string name, std::string descriptor)
    : m_access_flags(access_flags), m_name(std::move(name)), m_descriptor(std::move(descriptor))
{
}

MemberInfo::MemberInfo(BinaryObjectBuffer &file_stream, const ClassFile *class_file)
{
  parse(file_stream, class_file);
}

MemberInfo &MemberInfo::parse(BinaryObjectBuffer &file_stream, const ClassFile *class_file) noexcept(false)
{
  uint16_t flags {};
  file_stream.read_obj(flags);
  m_access_flags = flags;

  uint16_t name_index {};
  file_stream.read_obj(name_index);
  class_file->readConstant(name_index, m_name);

  uint16_t descriptor_index {};
  file_stream.read_obj(descriptor_index);
  class_file->readConstant(descriptor_index, m_descriptor);

  uint16_t attrib_count {};
  file_stream.read_obj(attrib_count);
  for (size_t ai = 0; ai < attrib_count; ai++)
  {
    m_attributes.emplace_back(file_stream, class_file);
  }

  return *this;
}
