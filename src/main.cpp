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

#include <map>
#include <string>
#include <fstream>

#include "JarFile.hpp"
#include "class/ClassFile.hpp"
#include "util/string.hpp"
#include <docopt/docopt.h>
#include <spdlog/spdlog.h>

static constexpr auto USAGE =
    R"(Lunar Decompiler.

    Usage:
          ldecomp <files> [-l files]
          ldecomp (-h | --help)
          ldecomp --version

    Files:
          Represents a semicolon (;) separated list of files. Both absolute
          and relative paths are allowed.

  Options:
          -l --lib      Libraries to include in decompilation context.
          -h --help     Show this screen.
          --version     Show version.
)";

int main(int argc, const char **argv)
{
  std::map<std::string, docopt::value> args =
      docopt::docopt(USAGE, {std::next(argv), std::next(argv, argc)}, true, "Lunar Decompiler 0.0.1");

#ifdef DEBUG
  spdlog::set_level(spdlog::level::debug);
#endif

  for (const auto &it : util::string::split_string(args["<files>"].asString(), ';'))
  {
    if (it.ends_with(".jar")) {
      spdlog::info("Processing file: {}", it);
      auto jar = JarFile(it);

      /* // Parsing Manifest files.
      auto fs = jar.openTextFile("META-INF/MANIFEST.MF").value();

      std::string line;
      while (util::string::getline(fs, line))
      {
        if (line.empty())
        {
          continue;
        }
        auto tokens = util::string::split_string(line, ':');

        spdlog::info("{}: {}", util::string::trim_copy(tokens[0]), util::string::trim_copy(tokens[1]));
      }
      */

      for (const auto &file_name : jar.files())
      {
        if (file_name.ends_with(".class"))
        {
          spdlog::info("\t- {}", file_name);
          auto cf = jar.openBinaryFile(file_name).value();
          ClassFile().parse(cf);
        }
      }
    } else if (it.ends_with(".class")) {
      spdlog::info("- {}", it);
      std::ifstream content(it);
      std::string content_string((std::istreambuf_iterator<char>(content)),
                      std::istreambuf_iterator<char>());
      auto cf = util::IObjStream(content_string);
      ClassFile().parse(cf);
    } else {
      spdlog::error("File must end with '.jar' or '.class'!");
    }
  }
}
