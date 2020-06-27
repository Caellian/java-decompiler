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

#include "../util/BinaryObjectBuffer.hpp"
#include "ConstantData.hpp"
#include "constant_tag.hpp"
#include "method_handle.hpp"
#include <memory>
#include <vector>

struct ConstantData;

class ConstantInfo
{
  constant_tag m_tag {};
  std::unique_ptr<ConstantData> m_dataptr;

public:
  ConstantInfo() noexcept = default;
  explicit ConstantInfo(BinaryObjectBuffer &file_stream) noexcept(false);
  ConstantInfo(const ConstantInfo &other) noexcept;
  ConstantInfo(ConstantInfo &&other) noexcept;
  ~ConstantInfo() noexcept = default;

  ConstantInfo &operator=(const ConstantInfo &other) noexcept;
  ConstantInfo &operator=(ConstantInfo &&other) noexcept;

  ConstantInfo &parse(BinaryObjectBuffer &file_stream) noexcept(false);

  [[nodiscard]] constant_tag tag() const
  {
    return m_tag;
  }
  [[nodiscard]] const ConstantData *data() const
  {
    return m_dataptr.get();
  }
  [[nodiscard]] std::string string() const
  {
    return constant_tag_name(m_tag) + " (" + m_dataptr->string() + ")";
  }
};

#endif // LDECOMP_CONSTANTINFO_HPP
