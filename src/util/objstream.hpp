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

#ifndef LDECOMP_OBJSTREAM_HPP
#define LDECOMP_OBJSTREAM_HPP

#include "endian.hpp"
#include <istream>
#include <sstream>

namespace util
{
template <typename CharType, typename CharTraits, typename AllocatorType>
class IObjStreamBase : public std::basic_istream<CharType, CharTraits>
{
public:
  // Types:
  using char_type = CharType;
  using traits_type = CharTraits;
  using allocator_type = AllocatorType;
  using int_type = typename traits_type::int_type;
  using pos_type = typename traits_type::pos_type;
  using off_type = typename traits_type::off_type;
  using string_type = std::basic_string<CharType, CharTraits, AllocatorType>;
  using stringbuf_type = std::basic_stringbuf<CharType, CharTraits, AllocatorType>;
  using istream_type = std::basic_istream<CharType, traits_type>;
  using this_type = IObjStreamBase<CharType, CharTraits, AllocatorType>;

private:
  stringbuf_type stringbuf;

public:
  IObjStreamBase() : istream_type(), stringbuf(std::ios_base::in)
  {
    this->init(&stringbuf);
  }
  explicit IObjStreamBase(std::ios_base::openmode mode) : istream_type(), stringbuf(mode | std::ios_base::in)
  {
    this->init(&stringbuf);
  }
  explicit IObjStreamBase(const string_type &str, std::ios_base::openmode mode = std::ios_base::in)
      : istream_type(), stringbuf(str, mode | std::ios_base::in)
  {
    this->init(&stringbuf);
  }

  ~IObjStreamBase() override = default;
  IObjStreamBase(const IObjStreamBase &) = delete;
  IObjStreamBase(IObjStreamBase &&other) noexcept
      : istream_type(std::move(other)), stringbuf(std::move(other.stringbuf))
  {
    istream_type::set_rdbuf(&stringbuf);
  }

  IObjStreamBase &operator=(const IObjStreamBase &) = delete;
  IObjStreamBase &operator=(IObjStreamBase &&other) noexcept
  {
    stringbuf = std::move(other.stringbuf);
    istream_type::operator=(std::move(other));
    return *this;
  }

  void swap(IObjStreamBase &other)
  {
    stringbuf.swap(other.stringbuf);
    istream_type::swap(other);
  }

  stringbuf_type *rdbuf() const
  {
    return const_cast<stringbuf_type *>(&stringbuf);
  }

  string_type str() const
  {
    return stringbuf.str();
  }

  void str(const string_type &string)
  {
    stringbuf.str(string);
  }

  IObjStreamBase<char_type, traits_type, allocator_type> &read(uint8_t *data, std::streamsize data_size)
  {
    std::istream::sentry se(*this, true);

    if (se)
    {
      std::ios_base::iostate err = std::ios_base::goodbit;
      try
      {
        auto readn = this->rdbuf()->sgetn(reinterpret_cast<char_type *>(data), data_size);
        if (readn != data_size)
        {
          err |= (std::ios_base::eofbit | std::ios_base::failbit);
        }
      }
      catch (...)
      {
        this->setstate(std::ios_base::badbit);
      }
      if (err)
      {
        this->setstate(err);
      }
    }
    return *this;
  }

  template <typename ObjectType>
  IObjStreamBase<char_type, traits_type, allocator_type> &read(ObjectType &obj,
                                                               std::endian expected_endianness = std::endian::big)
  {
    std::istream::sentry se(*this, true);

    if (se)
    {
      std::ios_base::iostate err = std::ios_base::goodbit;
      try
      {
        auto readn = this->rdbuf()->sgetn(reinterpret_cast<char_type *>(&obj), sizeof(obj));
        if (readn != sizeof(obj))
        {
          err |= (std::ios_base::eofbit | std::ios_base::failbit);
        }
        else if (expected_endianness != std::endian::native)
        {
          obj = util::endian::reverse(obj);
        }
      }
      catch (...)
      {
        this->setstate(std::ios_base::badbit);
      }
      if (err)
      {
        this->setstate(err);
      }
    }
    return *this;
  }

  IObjStreamBase<char_type, traits_type, allocator_type> &read(
      std::basic_string<char_type, traits_type, allocator_type> &string, std::streamsize len)
  {
    std::istream::sentry se(*this, true);

    if (se)
    {
      std::ios_base::iostate err = std::ios_base::goodbit;
      try
      {
        auto readn = this->rdbuf()->sgetn(string.data(), len);
        if (readn != len)
        {
          err |= (std::ios_base::eofbit | std::ios_base::failbit);
        }
      }
      catch (...)
      {
        this->setstate(std::ios_base::badbit);
      }
      if (static_cast<bool>(err))
      {
        this->setstate(err);
      }
    }
    return *this;
  }
};

using IObjStream = IObjStreamBase<char, std::char_traits<char>, std::allocator<char>>;

} // namespace util

#endif // LDECOMP_OBJSTREAM_HPP
