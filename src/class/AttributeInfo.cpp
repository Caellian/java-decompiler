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

#include "AttributeInfo.hpp"
#include <cstring>
#include <utility>

AttributeInfo::AttributeInfo(std::string name, uint32_t data_size, const uint8_t *data)
    : m_name(std::move(name)), m_size(data_size)
{
  m_data = new uint8_t[m_size];
  memcpy(m_data, data, m_size);
}

AttributeInfo::AttributeInfo(BinaryObjectBuffer &file_stream, const ClassFile *class_file)
{
  parse(file_stream, class_file);
}

AttributeInfo &AttributeInfo::parse(BinaryObjectBuffer &file_stream, const ClassFile *class_file) noexcept(false)
{
  uint16_t name_index {};
  file_stream.read_obj(name_index);
  class_file->readConstant(name_index, m_name);

  file_stream.read_obj(m_size);

  delete[] m_data; // delete old data if it's stored
  m_data = new uint8_t[m_size];
  file_stream.read(m_data, m_size);

  return *this;
}

AttributeInfo::~AttributeInfo() noexcept
{
  delete[] m_data;
}

AttributeInfo::AttributeInfo(AttributeInfo &&other) noexcept
    : m_name(std::move(other.m_name)), m_size(other.m_size), m_data(other.m_data)
{
  other.m_data = nullptr;
}

AttributeInfo::AttributeInfo(const AttributeInfo &other) noexcept
    : m_name(other.m_name), m_size(other.m_size), m_data(new uint8_t[other.m_size])
{
  std::memcpy(m_data, other.m_data, m_size);
}

AttributeInfo &AttributeInfo::operator=(const AttributeInfo &other) noexcept
{
  if (this != &other)
  {
    m_name = other.m_name;
    m_size = other.m_size;
    delete[] m_data;
    m_data = new uint8_t[m_size];
    std::memcpy(m_data, other.m_data, m_size);
  }

  return *this;
}

AttributeInfo &AttributeInfo::operator=(AttributeInfo &&other) noexcept
{
  if (this != &other)
  {
    m_name = std::move(other.m_name);
    m_size = other.m_size;
    delete[] m_data;
    m_data = other.m_data;
    other.m_data = nullptr;
  }

  return *this;
}
