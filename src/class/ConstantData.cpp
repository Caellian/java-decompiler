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

ConstantDataUtf8::ConstantDataUtf8(util::IObjStream &file_stream)
{
  uint16_t length {};
  file_stream.read(length);
  m_value.resize(length);
  file_stream.read(m_value, length);
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

ConstantDataMethodHandle::ConstantDataMethodHandle(util::IObjStream &file_stream)
{
  uint8_t kindRead {};
  file_stream.read(kindRead);
  m_ref_kind = static_cast<method_handle_kind>(kindRead);
  file_stream.read(m_ref_index);
}

ConstantDataReference::ConstantDataReference(util::IObjStream &file_stream)
{
  file_stream.read(this->m_value);
}

ConstantDataTypeSpec::ConstantDataTypeSpec(util::IObjStream &file_stream)
{
  file_stream.read(m_class_index);
  file_stream.read(m_name_and_type_index);
}

ConstantDataDescriptor::ConstantDataDescriptor(util::IObjStream &file_stream)
{
  file_stream.read(m_name_index);
  file_stream.read(m_descriptor_index);
}

ConstantDataDynamic::ConstantDataDynamic(util::IObjStream &file_stream)
{
  file_stream.read(m_bootstrap_method_attr_index);
  file_stream.read(m_name_and_type_index);
}
