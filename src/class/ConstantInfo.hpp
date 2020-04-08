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

#ifndef LDECOMP_CONSTANTINFO_HPP
#define LDECOMP_CONSTANTINFO_HPP

#include "../util/objstream.hpp"
#include "method_handle.hpp"
#include <memory>
#include <vector>

enum class constant_tag
{
  Utf8 = 1,
  // 2?
  Integer = 3,
  Float = 4,
  Long = 5,
  Double = 6,
  Class = 7,
  String = 8,
  FieldReference = 9,
  MethodReference = 10,
  InterfaceMethodReference = 11,
  NameAndType = 12,
  // 13?
  // 14?
  MethodHandle = 15,
  MethodType = 16,
  Dynamic = 17,
  InvokeDynamic = 18,
  Module = 19,
  Package = 20
};

class ConstantInfo;

struct ConstantInfoData
{
  ConstantInfoData() noexcept = default;
  ConstantInfoData(ConstantInfoData &&) noexcept = default;
  ConstantInfoData(const ConstantInfoData &) noexcept = default;
  virtual ~ConstantInfoData() noexcept = default;

  ConstantInfoData &operator=(const ConstantInfoData &other) = default;
  ConstantInfoData &operator=(ConstantInfoData &&other) = default;
};

template <typename WrappedType> class ConstantInfoDataWrapper : public ConstantInfoData
{
protected:
  WrappedType m_value {}; // NOLINT(misc-non-private-member-variables-in-classes)
public:
  [[nodiscard]] const WrappedType &value() const
  {
    return m_value;
  }
};

class ConstantInfoDataUtf8 : public ConstantInfoDataWrapper<std::string>
{
public:
  explicit ConstantInfoDataUtf8(util::IObjStream &file_stream)
  {
    uint16_t length {};
    file_stream.read(length);
    m_value.resize(length);
    file_stream.read(m_value, length);
  };
};

template <typename NumericType> class ConstantInfoDataNum : public ConstantInfoDataWrapper<NumericType>
{
public:
  explicit ConstantInfoDataNum(util::IObjStream &file_stream)
  {
    file_stream.read(this->m_value);
  }
};

class ConstantInfoDataReference : public ConstantInfoDataWrapper<uint16_t>
{
public:
  explicit ConstantInfoDataReference(util::IObjStream &file_stream)
  {
    file_stream.read(this->m_value);
  }

  template <typename DataStructure>
  DataStructure &accessReference(const std::vector<std::unique_ptr<ConstantInfo>> &const_pool) const;
};

class ConstantInfoDataRef : public ConstantInfoData
{
  uint16_t m_class_index {};
  uint16_t m_name_and_type_index {};

public:
  explicit ConstantInfoDataRef(util::IObjStream &file_stream)
  {
    file_stream.read(m_class_index);
    file_stream.read(m_name_and_type_index);
  }

  [[nodiscard]] uint16_t classIndex() const
  {
    return m_class_index;
  }
  [[nodiscard]] uint16_t nameAndTypeIndex() const
  {
    return m_name_and_type_index;
  }
};

class ConstantInfoDataNameAndRef : public ConstantInfoData
{
  uint16_t m_name_index {};
  uint16_t m_descriptor_index {};

public:
  explicit ConstantInfoDataNameAndRef(util::IObjStream &file_stream)
  {
    file_stream.read(m_name_index);
    file_stream.read(m_descriptor_index);
  }

  [[nodiscard]] uint16_t nameIndex() const
  {
    return m_name_index;
  }
  [[nodiscard]] uint16_t descriptorIndex() const
  {
    return m_descriptor_index;
  }
};

class ConstantInfoDataMethodHandle : public ConstantInfoData
{
  MethodHandleKind m_ref_kind {};
  uint16_t m_ref_index {};

public:
  explicit ConstantInfoDataMethodHandle(util::IObjStream &file_stream)
  {
    uint8_t kindRead {};
    file_stream.read(kindRead);
    m_ref_kind = static_cast<MethodHandleKind>(kindRead);
    file_stream.read(m_ref_index);
  }

  [[nodiscard]] MethodHandleKind referenceKind() const
  {
    return m_ref_kind;
  }
  [[nodiscard]] uint16_t referenceIndex() const
  {
    return m_ref_index;
  }
};

class ConstantInfoDataDynamic : public ConstantInfoData
{
  uint16_t m_bootstrap_method_attr_index {};
  uint16_t m_name_and_type_index {};

public:
  explicit ConstantInfoDataDynamic(util::IObjStream &file_stream)
  {
    file_stream.read(m_bootstrap_method_attr_index);
    file_stream.read(m_name_and_type_index);
  }

  [[nodiscard]] uint16_t attributeIndex() const
  {
    return m_bootstrap_method_attr_index;
  }
  [[nodiscard]] uint16_t nameAndTypeIndex() const
  {
    return m_name_and_type_index;
  }
};

class ConstantInfo
{
  constant_tag m_tag {};
  std::unique_ptr<ConstantInfoData> m_dataptr;

public:
  ConstantInfo() noexcept = default;

  ConstantInfo &parse(util::IObjStream &file_stream) noexcept(false);

  [[nodiscard]] constant_tag tag() const
  {
    return m_tag;
  }
  [[nodiscard]] const std::unique_ptr<ConstantInfoData> &data() const
  {
    return m_dataptr;
  }
};

template <typename DataStructure>
DataStructure &ConstantInfoDataReference::accessReference(
    const std::vector<std::unique_ptr<ConstantInfo>> &const_pool) const
{
  return *dynamic_cast<DataStructure *>(const_pool[m_value - 1uL].get()->data().get());
}

#endif // LDECOMP_CONSTANTINFO_HPP
