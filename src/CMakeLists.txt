include_directories(${ldecomp_includes})
set(ldecomp_sources ${ldecomp_sources}
        JarFile.cpp
        JarFile.hpp
        util/string.hpp
        error/file_error.hpp
        class/ClassFile.cpp
        class/ClassFile.hpp
        util/endian.hpp error/jar_error.hpp error/class_format_error.hpp util/objstream.hpp class/ConstantInfo.cpp class/ConstantInfo.hpp class/method_handle.hpp class/access_flags.hpp class/MemberInfo.cpp class/MemberInfo.hpp class/AttributeInfo.cpp class/AttributeInfo.hpp)

set(ldecomp_libs ${ldecomp_libs} CONAN_PKG::docopt.cpp CONAN_PKG::fmt
        CONAN_PKG::spdlog CONAN_PKG::minizip)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(ldecomp_def ${ldecomp_def} -DDEBUG)
endif ()

add_executable(ldecomp main.cpp ${ldecomp_sources})
target_link_libraries(ldecomp PRIVATE project_options project_warnings ${ldecomp_libs})
target_compile_definitions(ldecomp PRIVATE ${ldecomp_def})
install(
        TARGETS ldecomp
        RUNTIME DESTINATION bin
        LIBRARY DESTINATION ${LIB_INSTALL_DIR}
        ARCHIVE DESTINATION ${LIB_INSTALL_DIR})
