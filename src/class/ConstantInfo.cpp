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
