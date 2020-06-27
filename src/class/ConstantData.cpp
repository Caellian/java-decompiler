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

#include "ConstantData.hpp"

ConstantDataUtf8::ConstantDataUtf8(BinaryObjectBuffer &file_stream)
{
  uint16_t length {};
  file_stream.read_obj(length);
  file_stream.read_string(m_value, length);
}

ConstantDataUtf8::ConstantDataUtf8(const ConstantDataUtf8 &other) noexcept : ConstantDataWrapper(other)
{
  m_value = other.m_value;
}

ConstantDataUtf8::ConstantDataUtf8(ConstantDataUtf8 &&other) noexcept
{
  m_value = std::move(other.m_value);
}

ConstantDataUtf8 &ConstantDataUtf8::operator=(const ConstantDataUtf8 &other) noexcept
{
  if (this != &other)
  {
    m_value = other.m_value;
  }
  return *this;
}

ConstantDataUtf8 &ConstantDataUtf8::operator=(ConstantDataUtf8 &&other) noexcept
{
  if (this != &other)
  {
    m_value = std::move(other.m_value);
  }
  return *this;
}

ConstantDataMethodHandle::ConstantDataMethodHandle(BinaryObjectBuffer &file_stream)
{
  uint8_t kindRead {};
  file_stream.read_obj(kindRead);
  m_ref_kind = static_cast<method_handle_kind>(kindRead);
  file_stream.read_obj(m_ref_index);
}

ConstantDataReference::ConstantDataReference(BinaryObjectBuffer &file_stream)
{
  file_stream.read_obj(this->m_value);
}

// These can be defined as for instance:
// ConstantDataTypeSpec::ConstantDataTypeSpec(BinaryObjectBuffer &file_stream):
//    m_class_index(file_stream.read_obj<uint16_t>()), m_name_and_type_index(file_stream.read_obj<uint16_t>()) {}
// but that would make their deserialization order member variable declaration dependant

ConstantDataTypeSpec::ConstantDataTypeSpec(BinaryObjectBuffer &file_stream)
{
  file_stream.read_obj(m_class_index);
  file_stream.read_obj(m_name_and_type_index);
}

ConstantDataDescriptor::ConstantDataDescriptor(BinaryObjectBuffer &file_stream)
{
  file_stream.read_obj(m_name_index);
  file_stream.read_obj(m_descriptor_index);
}

ConstantDataDynamic::ConstantDataDynamic(BinaryObjectBuffer &file_stream)
{
  file_stream.read_obj(m_bootstrap_method_attr_index);
  file_stream.read_obj(m_name_and_type_index);
}
