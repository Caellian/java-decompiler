cmake_minimum_required(VERSION 3.15)
project(ldecomp CXX)

include(cmake/StandardProjectSettings.cmake)

add_library(project_options INTERFACE)
target_compile_features(project_options INTERFACE cxx_std_20)

add_library(project_warnings INTERFACE)
include(cmake/CompilerWarnings.cmake)
set_project_warnings(project_warnings)

include(cmake/Sanitizers.cmake)
enable_sanitizers(project_options)

include(cmake/Doxygen.cmake)
enable_doxygen()

include(cmake/StaticAnalyzers.cmake)

option(BUILD_SHARED_LIBS "Enable compilation of shared libraries" OFF)

option(ENABLE_PCH "Enable Precompiled Headers" ON)
if(ENABLE_PCH)
  target_precompile_headers(project_options INTERFACE <vector> <string> <map> <utility> <memory>)
endif()

set(CONAN_REQUIRES
        catch2/2.11.0
        docopt.cpp/0.6.2
        fmt/6.1.2
        spdlog/1.5.0
        minizip/1.2.11)
set(CONAN_OPTIONS "")
include(cmake/Conan.cmake)
run_conan()

# Include manually added third party libraries
add_subdirectory(thirdparty)

option(ENABLE_TESTING "Enable Test Builds" OFF)
if(ENABLE_TESTING)
  enable_testing()
  message("Building Tests")
  add_subdirectory(test)
endif()

option(ENABLE_FUZZING "Enable Fuzz Test Builds" OFF)
if(ENABLE_FUZZING)
  # https://www.llvm.org/docs/LibFuzzer.html
  message("Building Fuzz Tests")
  add_subdirectory(fuzz_test)
endif()

add_subdirectory(src)
