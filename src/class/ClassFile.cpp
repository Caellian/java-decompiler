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
#include "constant_tag.hpp"
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

  file_stream.read(m_constant_pool_size);
  m_constant_pool_size = uint16_t(m_constant_pool_size - 1);
  delete[] m_constant_pool; // Delete contents just in case parse has been called multiple times.
  m_constant_pool = new ConstantInfo[m_constant_pool_size];
  for (uint16_t i = 0, size_incr; i < m_constant_pool_size; i = uint16_t(i + size_incr))
  {
    m_constant_pool[i] = ConstantInfo();
    try
    {
      m_constant_pool[i].parse(file_stream);
    }
    catch (const class_format_error &cfe)
    {
      spdlog::warn("Error parsing class constant: {}", cfe.what());
      m_constant_pool[i] = ConstantInfo();
      spdlog::debug("Pushed placeholder dummy in constant pool.");
      file_stream.offset(-1);
    }
    size_incr = constant_tag_offset(m_constant_pool[i].tag());
  }

  uint16_t flags {};
  file_stream.read(flags);
  m_access_flags = flags;

  uint16_t this_index {};
  file_stream.read(this_index);
  if (this_index != 0) // should always be true
  {
    auto &this_class_entry = m_constant_pool[this_index - 1UL];
    if (this_class_entry.tag() != constant_tag::Class)
    {
      throw class_format_error("class field 'this' does not point to a class constant pool tag");
    }
    auto this_name_ref = dynamic_cast<const ConstantDataReference *>(this_class_entry.data());
    readConstant(this_name_ref, m_this_name);
  }
  else
  {
    throw class_format_error("class name not specified");
  }

  uint16_t super_index {};
  file_stream.read(super_index);
  if (super_index != 0)
  { // class has super
    auto &super_class_entry = m_constant_pool[super_index - 1UL];
    if (super_class_entry.tag() != constant_tag::Class)
    {
      throw class_format_error("class field 'super' does not point to a class constant pool tag");
    }
    auto super_name_ref = dynamic_cast<const ConstantDataReference *>(super_class_entry.data());
    readConstant(super_name_ref, m_super_name);
  }

  uint16_t interface_count {};
  file_stream.read(interface_count);
  m_interfaces.reserve(interface_count);
  for (uint16_t i = 0; i < interface_count; ++i)
  {
    uint16_t interface_index {};
    file_stream.read(interface_index);
    if (interface_index != 0)
    { // reference is valid
      auto &interface_class_entry = m_constant_pool[interface_index - 1UL];
      if (interface_class_entry.tag() != constant_tag::Class)
      {
        throw class_format_error("class interface does not point to a class constant pool tag");
      }
      auto interface_name_ref = dynamic_cast<const ConstantDataReference *>(interface_class_entry.data());
      std::string name;
      readConstant(interface_name_ref, name);
    }
    else
    {
      throw class_format_error("interface constant pool index is 0");
    }
  }

  uint16_t field_count {};
  file_stream.read(field_count);
  m_fields.reserve(field_count);
  for (uint16_t i = 0; i < field_count; ++i)
  {
    m_fields.push_back(MemberInfo().parse(file_stream, this));
  }

  uint16_t method_count {};
  file_stream.read(method_count);
  m_methods.reserve(method_count);
  for (uint16_t i = 0; i < method_count; ++i)
  {
    m_methods.push_back(MemberInfo().parse(file_stream, this));
  }

  uint16_t attrib_count {};
  file_stream.read(attrib_count);
  m_attributes.reserve(attrib_count);
  for (size_t i = 0; i < attrib_count; i++)
  {
    m_attributes.push_back(AttributeInfo().parse(file_stream, this));
  }

  return *this;
}

ClassFile::~ClassFile() noexcept
{
  delete[] m_constant_pool;
}

ClassFile::ClassFile(const ClassFile &other) noexcept :
  m_minor_version(other.m_minor_version),
  m_major_version(other.m_major_version),
  m_constant_pool_size(other.m_constant_pool_size),
  m_access_flags(other.m_access_flags),

  m_this_name(other.m_this_name),
  m_super_name(other.m_super_name),

  m_interfaces(other.m_interfaces),
  m_fields(other.m_fields),
  m_methods(other.m_methods),
  m_attributes(other.m_attributes)
{
  m_constant_pool = new ConstantInfo[m_constant_pool_size];
  for (size_t i = 0; i < m_constant_pool_size; ++i)
  {
    m_constant_pool[i] = other.m_constant_pool[i];
  }
}

ClassFile::ClassFile(ClassFile &&other) noexcept :
  m_minor_version(other.m_minor_version),
  m_major_version(other.m_major_version),
  m_constant_pool_size(other.m_constant_pool_size),
  m_constant_pool(other.m_constant_pool),
  m_access_flags(other.m_access_flags),

  m_this_name(std::move(other.m_this_name)),
  m_super_name(std::move(other.m_super_name)),

  m_interfaces(std::move(other.m_interfaces)),
  m_fields(std::move(other.m_fields)),
  m_methods(std::move(other.m_methods)),
  m_attributes(std::move(other.m_attributes))
{
  other.m_constant_pool = nullptr;
}

ClassFile &ClassFile::operator=(const ClassFile &other) noexcept
{
  if (this != &other)
  {
    m_minor_version = other.m_minor_version;
    m_major_version = other.m_major_version;
    m_constant_pool_size = other.m_constant_pool_size;
    m_constant_pool = new ConstantInfo[m_constant_pool_size];
    for (size_t i = 0; i < m_constant_pool_size; ++i)
    {
      m_constant_pool[i] = other.m_constant_pool[i];
    }
    m_access_flags = other.m_access_flags;

    m_this_name = other.m_this_name;
    m_super_name = other.m_super_name;

    m_interfaces = other.m_interfaces;
    m_fields = other.m_fields;
    m_methods = other.m_methods;
    m_attributes = other.m_attributes;
  }
  return *this;
}

ClassFile &ClassFile::operator=(ClassFile &&other) noexcept
{
  if (this != &other)
  {
    m_minor_version = other.m_minor_version;
    m_major_version = other.m_major_version;
    m_constant_pool_size = other.m_constant_pool_size;
    m_constant_pool = other.m_constant_pool;
    other.m_constant_pool = nullptr;
    m_access_flags = other.m_access_flags;

    m_this_name = std::move(other.m_this_name);
    m_super_name = std::move(other.m_super_name);

    m_interfaces = std::move(other.m_interfaces);
    m_fields = std::move(other.m_fields);
    m_methods = std::move(other.m_methods);
    m_attributes = std::move(other.m_attributes);
  }
  return *this;
}
