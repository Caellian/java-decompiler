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
#include "ConstantData.hpp"
#include "ConstantInfo.hpp"
#include "MemberInfo.hpp"
#include "tag_traits.hpp"

#include <bitset>
#include <vector>
#include <fmt/format.h>

class MemberInfo;
class AttributeInfo;

const size_t class_access_size = 16;

class ClassFile
{
  uint16_t m_minor_version {};
  uint16_t m_major_version {};
  uint16_t m_constant_pool_size {};
  ConstantInfo *m_constant_pool;
  std::bitset<class_access_size> m_access_flags {};

  std::string m_this_name {};
  std::string m_super_name {};

  std::vector<std::string> m_interfaces;
  std::vector<MemberInfo> m_fields;
  std::vector<MemberInfo> m_methods;
  std::vector<AttributeInfo> m_attributes;

public:
  ClassFile() noexcept = default;
  ~ClassFile() noexcept;

  ClassFile &parse(util::IObjStream &file_stream) noexcept(false);

  template <typename ReadType> void readConstant(uint16_t index, ReadType &into) const
  {
    index--;
    using traits = tag_primitive_traits<ReadType>;
    if (index < 0 || index > m_constant_pool_size)
    {
      throw std::logic_error(
          fmt::format("invalid constant pool index ({}), should be in range [1,{})", index, m_constant_pool_size));
    }
    if (m_constant_pool[index].tag() != tag_primitive_traits<ReadType>::tag)
    {
      throw std::runtime_error(fmt::format("invalid constant pool tag type ({}), expected {}",
                                           constant_tag_name(m_constant_pool[index].tag()),
                                           constant_tag_name(traits::tag)));
    }
    into = dynamic_cast<const typename traits::data_class *>(m_constant_pool[index].data())->value();
  }

  template <typename ReadType> void readConstant(const ConstantDataReference *ref, ReadType &into) const
  {
    if (ref == nullptr)
    {
      throw std::runtime_error("accessed reference is null");
    }
    auto index = ref->value();
    return readConstant(index, into);
  }

  [[nodiscard]] uint16_t minorVersion() const
  {
    return m_minor_version;
  };

  [[nodiscard]] uint16_t majorVersion() const
  {
    return m_major_version;
  };

  [[nodiscard]] uint16_t constantPoolSize() const
  {
    return m_constant_pool_size;
  };

  [[nodiscard]] const ConstantInfo *constantPool() const
  {
    return m_constant_pool;
  };

  [[nodiscard]] const std::bitset<class_access_size> &accessFlags() const
  {
    return m_access_flags;
  };

  [[nodiscard]] const std::string &thisName() const
  {
    return m_this_name;
  };

  [[nodiscard]] const std::string &superName() const
  {
    return m_super_name;
  };

  [[nodiscard]] const std::vector<std::string> &interfaces() const
  {
    return m_interfaces;
  };

  [[nodiscard]] const std::vector<MemberInfo> &fields() const
  {
    return m_fields;
  };

  [[nodiscard]] const std::vector<MemberInfo> &methods() const
  {
    return m_methods;
  };

  [[nodiscard]] const std::vector<AttributeInfo> &attributes() const
  {
    return m_attributes;
  };
};

#endif // LDECOMP_CLASSFILE_HPP