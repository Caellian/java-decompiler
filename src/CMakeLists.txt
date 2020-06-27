set(ldecomp_sources ${ldecomp_sources}
        JarFile.cpp
        JarFile.hpp
        class/AttributeInfo.cpp
        class/AttributeInfo.hpp
        class/ClassFile.cpp
        class/ClassFile.hpp
        class/ConstantData.cpp
        class/ConstantData.hpp
        class/ConstantInfo.cpp
        class/ConstantInfo.hpp
        class/MemberInfo.cpp
        class/MemberInfo.hpp
        class/access_flags.hpp
        class/constant_tag.cpp
        class/constant_tag.hpp
        class/method_handle.cpp
        class/method_handle.hpp
        class/tag_traits.hpp
        error/class_format_error.hpp
        error/file_error.hpp
        error/jar_error.hpp
        format/source_language.hpp
        util/BinaryObjectBuffer.cpp
        util/BinaryObjectBuffer.hpp
        util/endian.hpp
        util/string.cpp
        util/string.hpp
        )

set(ldecomp_libs ${ldecomp_libs}
        CONAN_PKG::docopt.cpp
        CONAN_PKG::fmt
        CONAN_PKG::spdlog
        CONAN_PKG::minizip
        )

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(ldecomp_def ${ldecomp_def} -DDEBUG)
endif ()

include(TestBigEndian)
TEST_BIG_ENDIAN(IS_BIG_ENDIAN)
if(IS_BIG_ENDIAN)
    set(ldecomp_def ${ldecomp_def} -DSYS_BIG_ENDIAN)
endif()

add_executable(ldecomp main.cpp ${ldecomp_sources})
target_link_libraries(ldecomp PRIVATE project_options project_warnings ${ldecomp_libs})

target_compile_definitions(ldecomp PRIVATE ${ldecomp_def})

target_include_directories(ldecomp PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/ldecomp>
        $<INSTALL_INTERFACE:include/ldecomp>)

install(
        TARGETS ldecomp
        RUNTIME DESTINATION bin
        LIBRARY DESTINATION ${LIB_INSTALL_DIR}
        ARCHIVE DESTINATION ${LIB_INSTALL_DIR})
