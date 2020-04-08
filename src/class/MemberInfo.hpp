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

#ifndef LDECOMP_MEMBERINFO_HPP
#define LDECOMP_MEMBERINFO_HPP

#include "../util/objstream.hpp"
#include "AttributeInfo.hpp"
#include "ConstantInfo.hpp"
#include <bitset>
#include <memory>
#include <string>

const size_t field_access_size = 16;

class MemberInfo
{
  std::bitset<field_access_size> m_access_flags {};
  std::string m_name;
  std::string m_descriptor;
  std::vector<AttributeInfo> m_attributes;

public:
  MemberInfo() noexcept = default;

  MemberInfo &parse(util::IObjStream &file_stream,
                    const std::vector<std::unique_ptr<ConstantInfo>> &const_pool) noexcept(false);

  [[nodiscard]] const std::bitset<field_access_size> &access_flags() const
  {
    return m_access_flags;
  };

  [[nodiscard]] const std::string &name() const
  {
    return m_name;
  };

  [[nodiscard]] const std::string &descriptor() const
  {
    return m_descriptor;
  };

  [[nodiscard]] const std::vector<AttributeInfo> &attributes() const
  {
    return m_attributes;
  };
};

#endif // LDECOMP_MEMBERINFO_HPP
