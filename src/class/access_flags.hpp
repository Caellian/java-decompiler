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

#ifndef LDECOMP_ACCESS_FLAGS_HPP
#define LDECOMP_ACCESS_FLAGS_HPP

enum class class_access_flag
{
  Public = 0,      // Declared public; may be accessed from outside its package.
  Final = 4,       // Declared final; no subclasses allowed.
  Super = 5,       // Treat superclass methods specially when invoked by the invokespecial instruction.
  Interface = 9,   // Is an interface, not a class.
  Abstract = 10,   // Declared abstract; must not be instantiated.
  Synthetic = 12,  // Declared synthetic; not present in the source code.
  Annotation = 13, // Declared as an annotation type.
  Enum = 14,       // Declared as an enum type.
  Module = 15      // Is a module, not a class or interface.
};

enum class field_access_flag
{
  Public = 0, // Declared public; may be accessed from outside its package.
  Private =
      1, // Declared private; accessible only within the defining class and other classes belonging to the same nest.
  Protected = 2,  // Declared protected; may be accessed within subclasses.
  Static = 3,     // Declared static; accessible without parent initialization.
  Final = 4,      // Declared final; never directly assigned to after object construction.
  Volatile = 6,   // Declared volatile; cannot be cached.
  Transient = 7,  // Declared transient; not written or read by a persistent object manager.
  Synthetic = 12, // Declared synthetic; not present in the source code.
  Enum = 14,      // Declared as an element of an enum.
};

#endif // LDECOMP_ACCESS_FLAGS_HPP
