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

#ifndef LDECOMP_ATTRIBUTEINFO_HPP
#define LDECOMP_ATTRIBUTEINFO_HPP

#include "../util/objstream.hpp"
#include "ConstantInfo.hpp"
#include <memory>
#include <string>
#include <vector>

class AttributeInfo
{
  std::string m_name;
  uint32_t m_size {};
  uint8_t *m_data = nullptr;

public:
  AttributeInfo() = default;
  AttributeInfo(AttributeInfo &&other) noexcept;
  AttributeInfo(const AttributeInfo &other) noexcept;
  ~AttributeInfo() noexcept;

  AttributeInfo &operator=(const AttributeInfo &other) noexcept;
  AttributeInfo &operator=(AttributeInfo &&other) noexcept;

  AttributeInfo &parse(util::IObjStream &file_stream,
                       const std::vector<std::unique_ptr<ConstantInfo>> &const_pool) noexcept(false);

  [[nodiscard]] const std::string &name() const
  {
    return m_name;
  };

  [[nodiscard]] uint32_t size() const
  {
    return m_size;
  };

  [[nodiscard]] const uint8_t *data() const
  {
    return m_data;
  };
};

#endif // LDECOMP_ATTRIBUTEINFO_HPP
