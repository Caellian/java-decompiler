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

#ifndef LDECOMP_CLASSFILE_HPP
#define LDECOMP_CLASSFILE_HPP

#include "../util/objstream.hpp"
#include "AttributeInfo.hpp"
#include "ConstantInfo.hpp"
#include "MemberInfo.hpp"

#include <bitset>
#include <vector>

const size_t class_access_size = 16;

class ClassFile
{
  uint16_t m_minor_version {};
  uint16_t m_major_version {};
  std::vector<std::unique_ptr<ConstantInfo>> m_constant_pool;
  std::bitset<class_access_size> m_access_flags {};

  std::string m_this_name {};
  std::string m_super_name {};

  std::vector<std::string> m_interfaces;
  std::vector<MemberInfo> m_fields;
  std::vector<MemberInfo> m_methods;
  std::vector<AttributeInfo> m_attributes;

public:
  ClassFile() noexcept = default;

  ClassFile &parse(util::IObjStream &file_stream) noexcept(false);
};

#endif // LDECOMP_CLASSFILE_HPP