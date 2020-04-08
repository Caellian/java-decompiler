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

MemberInfo &MemberInfo::parse(util::IObjStream &file_stream,
                              const std::vector<std::unique_ptr<ConstantInfo>> &const_pool) noexcept(false)
{
  uint16_t flags {};
  file_stream.read(flags);
  m_access_flags = flags;

  uint16_t name_index {};
  file_stream.read(name_index);
  m_name = dynamic_cast<ConstantInfoDataUtf8 *>(const_pool[name_index - 1uL]->data().get())->value();

  uint16_t descriptor_index {};
  file_stream.read(descriptor_index);
  m_descriptor = dynamic_cast<ConstantInfoDataUtf8 *>(const_pool[descriptor_index - 1uL]->data().get())->value();

  uint16_t attrib_count {};
  file_stream.read(attrib_count);
  for (size_t ai = 0; ai < attrib_count; ai++)
  {
    m_attributes.push_back(AttributeInfo().parse(file_stream, const_pool));
  }

  return *this;
}
