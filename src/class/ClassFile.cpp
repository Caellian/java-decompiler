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

#include "ClassFile.hpp"

#include "../error/class_format_error.hpp"
#include "../util/endian.hpp"
#include <spdlog/spdlog.h>

const uint32_t class_signature = 0xCAFEBABE;

bool test_magic_number(util::IObjStream &file)
{
  uint32_t magic = 0;
  file.read(magic);
  return magic == class_signature;
}

ClassFile &ClassFile::parse(util::IObjStream &file_stream) noexcept(false)
{
  if (!test_magic_number(file_stream))
  {
    throw class_format_error("invalid magic number");
  }

  file_stream.read(m_minor_version);
  file_stream.read(m_major_version);

  uint16_t constant_pool_count = 0;
  file_stream.read(constant_pool_count);
  for (uint16_t i = 1; i < constant_pool_count; ++i)
  {
    auto ptr = std::make_unique<ConstantInfo>();
    ptr->parse(file_stream);
    m_constant_pool.push_back(std::move(ptr));
  }

  uint16_t flags {};
  file_stream.read(flags);
  m_access_flags = flags;

  uint16_t this_index {};
  file_stream.read(this_index);
  if (this_index != 0) // should always be true
  {
    auto &this_class_entry = m_constant_pool[this_index - 1uL];
    if (this_class_entry->tag() != constant_tag::Class)
    {
      throw class_format_error("class field 'this' does not point to a class constant pool tag");
    }
    auto this_name_ref = dynamic_cast<ConstantInfoDataReference *>(this_class_entry->data().get());
    if (this_name_ref != nullptr)
    {
      auto utf8_field = this_name_ref->accessReference<ConstantInfoDataUtf8>(m_constant_pool);
      m_this_name = utf8_field.value();
    }
  }

  uint16_t super_index {};
  file_stream.read(super_index);
  if (super_index != 0)
  { // class has super
    auto &super_class_entry = m_constant_pool[super_index - 1uL];
    if (super_class_entry->tag() != constant_tag::Class)
    {
      throw class_format_error("class field 'super' does not point to a class constant pool tag");
    }
    auto super_name_ref = dynamic_cast<ConstantInfoDataReference *>(super_class_entry->data().get());
    if (super_name_ref != nullptr)
    {
      auto utf8_field = super_name_ref->accessReference<ConstantInfoDataUtf8>(m_constant_pool);
      m_super_name = utf8_field.value();
    }
  }

  uint16_t interface_count {};
  file_stream.read(interface_count);
  for (uint16_t i = 0; i < interface_count; ++i)
  {
    uint16_t interface_index {};
    file_stream.read(interface_index);
    if (interface_index != 0)
    { // reference is valid
      auto &interface_class_entry = m_constant_pool[interface_index - 1uL];
      if (interface_class_entry->tag() != constant_tag::Class)
      {
        throw class_format_error("class interface does not point to a class constant pool tag");
      }
      auto interface_name_ref = dynamic_cast<ConstantInfoDataReference *>(interface_class_entry->data().get());
      if (interface_name_ref != nullptr)
      {
        auto utf8_field = interface_name_ref->accessReference<ConstantInfoDataUtf8>(m_constant_pool);
        m_interfaces.push_back(utf8_field.value());
      }
    }
  }

  uint16_t field_count {};
  file_stream.read(field_count);
  for (uint16_t i = 0; i < field_count; ++i)
  {
    m_fields.push_back(MemberInfo().parse(file_stream, m_constant_pool));
  }

  uint16_t method_count {};
  file_stream.read(method_count);
  for (uint16_t i = 0; i < method_count; ++i)
  {
    m_methods.push_back(MemberInfo().parse(file_stream, m_constant_pool));
  }

  uint16_t attrib_count {};
  file_stream.read(attrib_count);
  for (size_t i = 0; i < attrib_count; i++)
  {
    m_attributes.push_back(AttributeInfo().parse(file_stream, m_constant_pool));
  }

  return *this;
}
