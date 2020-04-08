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

#ifndef LDECOMP_METHOD_HANDLE_HPP
#define LDECOMP_METHOD_HANDLE_HPP

enum class MethodHandleKind
{
  GetField = 1,
  GetStatic = 2,
  PutField = 3,
  PutStatic = 4,
  InvokeVirtual = 5,
  InvokeStatic = 6,
  InvokeSpecial = 7,
  NewInvokeSpecial = 8,
  InvokeInterface = 9
};

#endif // LDECOMP_METHOD_HANDLE_HPP
