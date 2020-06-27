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

#ifndef LDECOMP_BINARYOBJECTBUFFER_HPP
#define LDECOMP_BINARYOBJECTBUFFER_HPP

#include "endian.hpp"
#include <bits/ios_base.h>
#include <cstdlib>

/**
 * Handles deserialization and serialization of binary objects.
 * Adjusts to system endianness.
 */
class BinaryObjectBuffer
{
  uint8_t *m_data;
  size_t m_size;
  bool m_can_grow = false;

  size_t m_pos = 0;

  util::endian::Endianness m_endianness;

public:
  explicit BinaryObjectBuffer(size_t size, util::endian::Endianness endianness = util::endian::Endianness::BigEndian);

  /// Creates a new object by copying data specified by arguments
  BinaryObjectBuffer(const uint8_t *data, size_t size,
                     util::endian::Endianness endianness = util::endian::Endianness::BigEndian);

  explicit BinaryObjectBuffer(std::ifstream &input,
                              util::endian::Endianness endianness = util::endian::Endianness::BigEndian);

  ~BinaryObjectBuffer();

  [[nodiscard]] uint8_t *data()
  {
    return m_data;
  }

  [[nodiscard]] size_t size() const
  {
    return m_size;
  }

  [[nodiscard]] bool can_grow() const
  {
    return m_can_grow;
  }

  bool can_grow(bool new_value);

  [[nodiscard]] size_t pos() const
  {
    return m_pos;
  }

  BinaryObjectBuffer &pos(size_t absolute);

  std::size_t offset(long relative);

  [[nodiscard]] bool has_next() const;

  [[nodiscard]] util::endian::Endianness endianness() const
  {
    return m_endianness;
  }

  util::endian::Endianness endianness(util::endian::Endianness new_endianness);

  BinaryObjectBuffer &read(uint8_t *data, size_t data_size, bool reverse = false);

  BinaryObjectBuffer &write(const uint8_t *source, size_t data_size, bool reverse = false);

  template <typename StringType> BinaryObjectBuffer &read_string(StringType &string, size_t length)
  {
    string.resize(length);
    read(reinterpret_cast<uint8_t *>(string.data()), length);

    return *this;
  }

  template <typename StringType> BinaryObjectBuffer &write_string(StringType &string, size_t length)
  {
    write(reinterpret_cast<uint8_t *>(string.data()), length);

    return *this;
  }

  template <typename ObjectType>[[nodiscard]] ObjectType read_obj()
  {
    auto obj = ObjectType {};

    read(reinterpret_cast<uint8_t *>(&obj), sizeof(ObjectType),
#ifdef SYS_BIG_ENDIAN
         m_endianness == util::endian::Endianness::LittleEndian);
#else
         m_endianness == util::endian::Endianness::BigEndian);
#endif

    return obj;
  }

  template <typename ObjectType> BinaryObjectBuffer &read_obj(ObjectType &obj)
  {
    read(reinterpret_cast<uint8_t *>(&obj), sizeof(ObjectType),
#ifdef SYS_BIG_ENDIAN
         m_endianness == util::endian::Endianness::LittleEndian);
#else
         m_endianness == util::endian::Endianness::BigEndian);
#endif

    return *this;
  }

  template <typename ObjectType> BinaryObjectBuffer &write_obj(const ObjectType &obj)
  {
    write(reinterpret_cast<uint8_t *>(&obj), sizeof(ObjectType),
#ifdef SYS_BIG_ENDIAN
          m_endianness == util::endian::Endianness::LittleEndian);
#else
          m_endianness == util::endian::Endianness::BigEndian);
#endif

    return *this;
  }
};

#endif // LDECOMP_BINARYOBJECTBUFFER_HPP
