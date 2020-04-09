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

#ifndef LDECOMP_CONSTANTDATA_HPP
#define LDECOMP_CONSTANTDATA_HPP

#include "../util/objstream.hpp"
#include "method_handle.hpp"

struct ConstantData
{
  ConstantData() noexcept = default;
  ConstantData(ConstantData &&) noexcept = default;
  ConstantData(const ConstantData &) noexcept = default;
  virtual ~ConstantData() noexcept = default;

  ConstantData &operator=(const ConstantData &other) = default;
  ConstantData &operator=(ConstantData &&other) = default;

  [[nodiscard]] virtual std::string string() const
  {
    return "unimplemented";
  }
};
template <typename WrappedType> class ConstantDataWrapper : public ConstantData
{
protected:
  WrappedType m_value {}; // NOLINT(misc-non-private-member-variables-in-classes)
public:
  [[nodiscard]] const WrappedType &value() const
  {
    return m_value;
  }
};
class ConstantDataUtf8 : public ConstantDataWrapper<std::string>
{
public:
  explicit ConstantDataUtf8(util::IObjStream &file_stream);

  [[nodiscard]] std::string string() const override
  {
    return "'" + m_value + "'";
  }
};

template <typename NumericType> class ConstantDataNumeric : public ConstantDataWrapper<NumericType>
{
public:
  explicit ConstantDataNumeric(util::IObjStream &file_stream)
  {
    file_stream.read(this->m_value);
  }

  [[nodiscard]] virtual std::string string() const
  {
    return std::to_string(this->m_value);
  }
};

class ConstantDataReference : public ConstantDataWrapper<uint16_t>
{
public:
  explicit ConstantDataReference(util::IObjStream &file_stream);

  [[nodiscard]] std::string string() const override
  {
    return "index: #" + std::to_string(m_value);
  }
};

class ConstantDataTypeSpec : public ConstantData
{
  uint16_t m_class_index {};
  uint16_t m_name_and_type_index {};

public:
  explicit ConstantDataTypeSpec(util::IObjStream &file_stream);

  [[nodiscard]] uint16_t classIndex() const
  {
    return m_class_index;
  }
  [[nodiscard]] uint16_t nameAndTypeIndex() const
  {
    return m_name_and_type_index;
  }

  [[nodiscard]] std::string string() const override
  {
    return "class: #" + std::to_string(m_class_index) + ", name and type: #" + std::to_string(m_name_and_type_index);
  }
};
class ConstantDataDescriptor : public ConstantData
{
  uint16_t m_name_index {};
  uint16_t m_descriptor_index {};

public:
  explicit ConstantDataDescriptor(util::IObjStream &file_stream);

  [[nodiscard]] uint16_t nameIndex() const
  {
    return m_name_index;
  }
  [[nodiscard]] uint16_t descriptorIndex() const
  {
    return m_descriptor_index;
  }

  [[nodiscard]] std::string string() const override
  {
    return "name: #" + std::to_string(m_name_index) + ", descriptor: #" + std::to_string(m_descriptor_index);
  }
};

class ConstantDataMethodHandle : public ConstantData
{
  method_handle_kind m_ref_kind {};
  uint16_t m_ref_index {};

public:
  explicit ConstantDataMethodHandle(util::IObjStream &file_stream);

  [[nodiscard]] method_handle_kind referenceKind() const
  {
    return m_ref_kind;
  }
  [[nodiscard]] uint16_t referenceIndex() const
  {
    return m_ref_index;
  }

  [[nodiscard]] std::string string() const override
  {
    return "kind: " + method_handle_kind_name(m_ref_kind) + ", ref: #" + std::to_string(m_ref_index);
  }
};
class ConstantDataDynamic : public ConstantData
{
  uint16_t m_bootstrap_method_attr_index {};
  uint16_t m_name_and_type_index {};

public:
  explicit ConstantDataDynamic(util::IObjStream &file_stream);

  [[nodiscard]] uint16_t attributeIndex() const
  {
    return m_bootstrap_method_attr_index;
  }
  [[nodiscard]] uint16_t nameAndTypeIndex() const
  {
    return m_name_and_type_index;
  }

  [[nodiscard]] std::string string() const override
  {
    return "bootstrap mtd attr: #" + std::to_string(m_bootstrap_method_attr_index) + ", name and type: #" +
           std::to_string(m_name_and_type_index);
  }
};

#endif // LDECOMP_CONSTANTDATA_HPP
