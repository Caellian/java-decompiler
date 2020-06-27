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

#include <fstream>
#include <map>
#include <string>

#include "JarFile.hpp"
#include "class/ClassFile.hpp"
#include "util/string.hpp"
#include <docopt/docopt.h>
#include <spdlog/spdlog.h>

static constexpr auto USAGE = R"(
Lunar Decompiler.

    Usage:
          ldecomp [--log-level=LEVEL] <file>...
          ldecomp -h | --help
          ldecomp --version

    Files:
          Absolute or relative path to a .class or .jar file.

  Options:
          --log-level=LEVEL  Set logging level. Can be trace, debug, info, warn, error,
                             critical or off. [default: "info"]
          -i --index=OUTPUT  Generate decompiled class index
          -h --help          Show this screen and terminate the program.
          --version          Show version and terminate the program.
)";

int main(int argc, const char **argv) // temporary NOLINT(bugprone-exception-escape)
{
  std::map<std::string, docopt::value> args =
      docopt::docopt(USAGE, {std::next(argv), std::next(argv, argc)}, true, "Lunar Decompiler 0.0.1");

  std::string log_level = args["--log-level"].asString();
  if (log_level == "trace") {
    spdlog::set_level(spdlog::level::trace);
  } else if (log_level == "debug") {
    spdlog::set_level(spdlog::level::debug);
  } else if (log_level == "info") {
    spdlog::set_level(spdlog::level::info);
  } else if (log_level == "warn") {
    spdlog::set_level(spdlog::level::warn);
  } else if (log_level == "error") {
    spdlog::set_level(spdlog::level::err);
  } else if (log_level == "critical") {
    spdlog::set_level(spdlog::level::critical);
  } else if (log_level == "off") {
    spdlog::set_level(spdlog::level::off);
  }

  for (const auto &it : args["<file>"].asStringList())
  {
    if (util::string::ends_with(it, ".jar"))
    {
      spdlog::info("Processing file: {}", it);
      auto jar = JarFile(it);

      auto files = jar.files();
      auto iter = std::find_if(files.begin(), files.end(), [](const auto &f) {
        return util::string::ends_with(f, ".class");
      });

      while (iter != files.end())
      {
        spdlog::info("\t- {}", *iter);

        auto *class_stream = jar.openBinaryFile(*iter);
        if (class_stream == nullptr) {
          spdlog::warn("Unable to read {}", *iter);
          continue;
        }
        const auto cf = ClassFile(std::move(*class_stream));

        spdlog::info("Class: {}", cf.thisName());

        for (const auto &m : cf.methods())
        {
          spdlog::info("\tMethod: {}, desc: {}", m.name(), m.descriptor());
          for (const auto &a : m.attributes())
          {
            if (a.name() == "Code")
            {
              // Following lines are here for development purposes
              // There's a lot of linting errors which are being ignored
              for (uint32_t i = 0; i < a.size(); ++i)
              {
                printf("%02X ", a.data()[i]); // NOLINT(hicpp-vararg)

                if ((i + 1) % 16 == 0) // NOLINT(readability-magic-numbers)
                {
                  printf("\n"); // NOLINT(hicpp-vararg)
                }
                else if ((i + 1) % 4 == 0) // NOLINT(readability-magic-numbers)
                {
                  printf(" "); // NOLINT(hicpp-vararg)
                }
              }
              printf("\n"); // NOLINT(hicpp-vararg)
            }
          }
        }

        iter = std::find_if(++iter, files.end(), [](const auto &f) {
          return util::string::ends_with(f, ".class");
        });
      }
    }
    else if (util::string::ends_with(it, ".class"))
    {
      spdlog::info("- {}", it);

      std::ifstream input( it, std::ios::binary );
      if (input.is_open()) {
        spdlog::error("Unable to open: {}", it);
        continue;
      }
      auto cf = ClassFile(BinaryObjectBuffer(input));
    }
    else
    {
      spdlog::error("File must end with '.jar' or '.class'!");
    }
  }
}
