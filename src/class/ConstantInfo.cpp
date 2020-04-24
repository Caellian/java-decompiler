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

#include "ConstantInfo.hpp"
#include "../error/class_format_error.hpp"
#include "ConstantData.hpp"

#include <cstdlib>

ConstantInfo &ConstantInfo::parse(util::IObjStream &file_stream) noexcept(false)
{
  uint8_t tagVal {};
  file_stream.read(tagVal);
  m_tag = static_cast<constant_tag>(tagVal);

  switch (m_tag)
  {
  case constant_tag::Utf8:
    m_dataptr = std::make_unique<ConstantDataUtf8>(file_stream);
    break;
  case constant_tag::Integer:
    m_dataptr = std::make_unique<ConstantDataNumeric<int32_t>>(file_stream);
    break;
  case constant_tag::Float:
    m_dataptr = std::make_unique<ConstantDataNumeric<float>>(file_stream);
    break;
  case constant_tag::Long:
    m_dataptr = std::make_unique<ConstantDataNumeric<int64_t>>(file_stream);
    break;
  case constant_tag::Double:
    m_dataptr = std::make_unique<ConstantDataNumeric<double>>(file_stream);
    break;
  case constant_tag::FieldReference:
  case constant_tag::MethodReference:
  case constant_tag::InterfaceMethodReference:
    m_dataptr = std::make_unique<ConstantDataTypeSpec>(file_stream);
    break;
  case constant_tag::NameAndType:
    m_dataptr = std::make_unique<ConstantDataDescriptor>(file_stream);
    break;
  case constant_tag::MethodHandle:
    m_dataptr = std::make_unique<ConstantDataMethodHandle>(file_stream);
    break;
  case constant_tag::Dynamic:
  case constant_tag::InvokeDynamic:
    m_dataptr = std::make_unique<ConstantDataDynamic>(file_stream);
    break;
  case constant_tag::Class:
  case constant_tag::String:
  case constant_tag::MethodType:
  case constant_tag::Module:
  case constant_tag::Package:
    m_dataptr = std::make_unique<ConstantDataReference>(file_stream);
    break;
  default:
    throw class_format_error("invalid constant tag (tag: " + std::to_string(static_cast<uint8_t>(m_tag)) + ")");
  }

  return *this;
}

ConstantInfo::ConstantInfo(const ConstantInfo &other) noexcept : m_tag(other.m_tag)
{
  switch (m_tag)
  {
  case constant_tag::Utf8:
    m_dataptr = std::make_unique<ConstantDataUtf8>(dynamic_cast<ConstantDataUtf8 &>(*other.m_dataptr));
    break;
  case constant_tag::Integer:
    m_dataptr = std::make_unique<ConstantDataNumeric<int32_t>>(dynamic_cast<ConstantDataNumeric<int32_t> &>(*other.m_dataptr));
    break;
  case constant_tag::Float:
    m_dataptr = std::make_unique<ConstantDataNumeric<float>>(dynamic_cast<ConstantDataNumeric<float> &>(*other.m_dataptr));
    break;
  case constant_tag::Long:
    m_dataptr = std::make_unique<ConstantDataNumeric<int64_t>>(dynamic_cast<ConstantDataNumeric<int64_t> &>(*other.m_dataptr));
    break;
  case constant_tag::Double:
    m_dataptr = std::make_unique<ConstantDataNumeric<double>>(dynamic_cast<ConstantDataNumeric<double> &>(*other.m_dataptr));
    break;
  case constant_tag::FieldReference:
  case constant_tag::MethodReference:
  case constant_tag::InterfaceMethodReference:
    m_dataptr = std::make_unique<ConstantDataTypeSpec>(dynamic_cast<ConstantDataTypeSpec &>(*other.m_dataptr));
    break;
  case constant_tag::NameAndType:
    m_dataptr = std::make_unique<ConstantDataDescriptor>(dynamic_cast<ConstantDataDescriptor &>(*other.m_dataptr));
    break;
  case constant_tag::MethodHandle:
    m_dataptr = std::make_unique<ConstantDataMethodHandle>(dynamic_cast<ConstantDataMethodHandle &>(*other.m_dataptr));
    break;
  case constant_tag::Dynamic:
  case constant_tag::InvokeDynamic:
    m_dataptr = std::make_unique<ConstantDataDynamic>(dynamic_cast<ConstantDataDynamic &>(*other.m_dataptr));
    break;
  case constant_tag::Class:
  case constant_tag::String:
  case constant_tag::MethodType:
  case constant_tag::Module:
  case constant_tag::Package:
    m_dataptr = std::make_unique<ConstantDataReference>(dynamic_cast<ConstantDataReference &>(*other.m_dataptr));
  default:
    break;
  }
}

ConstantInfo::ConstantInfo(ConstantInfo &&other) noexcept : m_tag(other.m_tag), m_dataptr(std::move(other.m_dataptr))
{}

ConstantInfo &ConstantInfo::operator=(const ConstantInfo &other) noexcept
{
  if (this != &other)
  {
    m_tag = other.m_tag;
    switch (m_tag)
    {
    case constant_tag::Utf8:
      m_dataptr = std::make_unique<ConstantDataUtf8>(dynamic_cast<ConstantDataUtf8 &>(*other.m_dataptr));
      break;
    case constant_tag::Integer:
      m_dataptr = std::make_unique<ConstantDataNumeric<int32_t>>(dynamic_cast<ConstantDataNumeric<int32_t> &>(*other.m_dataptr));
      break;
    case constant_tag::Float:
      m_dataptr = std::make_unique<ConstantDataNumeric<float>>(dynamic_cast<ConstantDataNumeric<float> &>(*other.m_dataptr));
      break;
    case constant_tag::Long:
      m_dataptr = std::make_unique<ConstantDataNumeric<int64_t>>(dynamic_cast<ConstantDataNumeric<int64_t> &>(*other.m_dataptr));
      break;
    case constant_tag::Double:
      m_dataptr = std::make_unique<ConstantDataNumeric<double>>(dynamic_cast<ConstantDataNumeric<double> &>(*other.m_dataptr));
      break;
    case constant_tag::FieldReference:
    case constant_tag::MethodReference:
    case constant_tag::InterfaceMethodReference:
      m_dataptr = std::make_unique<ConstantDataTypeSpec>(dynamic_cast<ConstantDataTypeSpec &>(*other.m_dataptr));
      break;
    case constant_tag::NameAndType:
      m_dataptr = std::make_unique<ConstantDataDescriptor>(dynamic_cast<ConstantDataDescriptor &>(*other.m_dataptr));
      break;
    case constant_tag::MethodHandle:
      m_dataptr = std::make_unique<ConstantDataMethodHandle>(dynamic_cast<ConstantDataMethodHandle &>(*other.m_dataptr));
      break;
    case constant_tag::Dynamic:
    case constant_tag::InvokeDynamic:
      m_dataptr = std::make_unique<ConstantDataDynamic>(dynamic_cast<ConstantDataDynamic &>(*other.m_dataptr));
      break;
    case constant_tag::Class:
    case constant_tag::String:
    case constant_tag::MethodType:
    case constant_tag::Module:
    case constant_tag::Package:
      m_dataptr = std::make_unique<ConstantDataReference>(dynamic_cast<ConstantDataReference &>(*other.m_dataptr));
    default:
      break;
    }
  }
  return *this;
}

ConstantInfo &ConstantInfo::operator=(ConstantInfo &&other) noexcept
{
  if (this != &other)
  {
    m_tag = other.m_tag;
    m_dataptr = std::move(other.m_dataptr);
  }
  return *this;
}
