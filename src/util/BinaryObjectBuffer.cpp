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

#include "BinaryObjectBuffer.hpp"

#include <cmath>
#include <cstring>
#include <fstream>
#include <vector>

BinaryObjectBuffer::BinaryObjectBuffer(std::size_t size, util::endian::Endianness endianness)
    : m_size(size), m_endianness(endianness)
{
  m_data = new uint8_t[size];
}

BinaryObjectBuffer::BinaryObjectBuffer(const uint8_t *data, std::size_t size, util::endian::Endianness endianness)
    : BinaryObjectBuffer(size, endianness)
{
  std::memcpy(m_data, data, size);
}

BinaryObjectBuffer::BinaryObjectBuffer(std::ifstream &input, util::endian::Endianness endianness)
    : m_endianness(endianness)
{
  std::vector<uint8_t> buff(std::istreambuf_iterator<char>(input), {});

  m_data = new uint8_t[buff.size()];
  m_size = buff.size();
  std::memcpy(m_data, buff.data(), m_size);
}

BinaryObjectBuffer::~BinaryObjectBuffer()
{
  delete m_data;
}

BinaryObjectBuffer &BinaryObjectBuffer::pos(std::size_t absolute)
{
  m_pos = absolute;

  if (m_pos > m_size)
  {
    m_pos = m_size;
  }

  return *this;
}

std::size_t BinaryObjectBuffer::offset(long relative)
{
  return m_pos = std::min(
             static_cast<std::size_t>(std::max(relative < 0 ? m_pos - static_cast<std::size_t>(std::abs(relative))
                                                            : m_pos + static_cast<std::size_t>(relative),
                                               std::size_t {})),
             m_size);
}

bool BinaryObjectBuffer::has_next() const
{
  return m_pos < m_size;
}

bool BinaryObjectBuffer::can_grow(bool new_value)
{
  bool prev = m_can_grow;
  m_can_grow = new_value;
  return prev;
}

util::endian::Endianness BinaryObjectBuffer::endianness(util::endian::Endianness new_endianness)
{
  auto prev = m_endianness;
  m_endianness = new_endianness;
  return prev;
}

BinaryObjectBuffer &BinaryObjectBuffer::read(uint8_t *data, std::size_t data_size, bool reverse)
{
  if (m_pos + data_size > m_size)
  {
    throw std::runtime_error("tried reading data outside buffer bounds");
  }

  if (reverse)
  {
    for (std::size_t i = 0; i < data_size; ++i)
    {
      data[data_size - 1 - i] = m_data[m_pos + i];
    }
  }
  else
  {
    for (std::size_t i = 0; i < data_size; ++i)
    {
      data[i] = m_data[m_pos + i];
    }
  }
  m_pos += data_size;

  return *this;
}

BinaryObjectBuffer &BinaryObjectBuffer::write(const uint8_t *source, std::size_t data_size, bool reverse)
{
  if (m_pos + data_size > m_size)
  {
    throw std::runtime_error("tried writing data outside buffer bounds");
  }

  if (reverse)
  {
    for (std::size_t i = 0; i < data_size; ++i)
    {
      m_data[m_pos + i] = source[data_size - 1 - i];
    }
  }
  else
  {
    for (std::size_t i = 0; i < data_size; ++i)
    {
      m_data[m_pos + i] = source[i];
    }
  }
  m_pos += data_size;

  return *this;
}
