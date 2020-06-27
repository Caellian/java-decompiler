cmake_minimum_required(VERSION 3.13)
if (${CMAKE_BINARY_DIR} STREQUAL ${CMAKE_SOURCE_DIR} AND NOT "$ENV{CI_BUILD}")
  message(FATAL_ERROR "Stopping the in-source build.")
endif()

project(ldecomp CXX)

include(cmake/StandardProjectSettings.cmake)

add_library(project_options INTERFACE)
target_compile_features(project_options INTERFACE cxx_std_17)

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
        catch2/2.12.2
        docopt.cpp/0.6.2
        fmt/6.2.1
        spdlog/1.6.1
        minizip/1.2.11
        )
set(CONAN_OPTIONS "")
include(cmake/Conan.cmake)
run_conan()

# Include manually added third party libraries
add_subdirectory(library)

add_subdirectory(src)

option(ENABLE_TESTING "Enable Test Builds" OFF)
if(ENABLE_TESTING OR ENABLE_FUZZING)
  message(STATUS "Testing enabled")
  enable_testing()
endif()

if(ENABLE_TESTING)
  add_subdirectory(test)
  message(STATUS "Configuring tests done")
endif()

option(ENABLE_FUZZING "Enable Fuzz Test Builds" OFF)
if(ENABLE_FUZZING)
  add_subdirectory(fuzz_test)
  message(STATUS "Configuring fuzz tests done")
endif()
