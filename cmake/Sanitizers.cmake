function(enable_sanitizers project_name)

    if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
        option(ENABLE_COVERAGE "Enable coverage reporting for gcc/clang" FALSE)

        if (ENABLE_COVERAGE)
            target_compile_options(project_options INTERFACE --coverage -O0 -g)
            target_link_libraries(project_options INTERFACE --coverage)
        endif ()

        define_property(GLOBAL PROPERTY ENABLED_SANITIZER
                BRIEF_DOCS "Sanitizer to compile with"
                FULL_DOCS
                Allows specifying a sanitizer to include in binary.
                Useful to automated CI tests.
                )

        if (NOT ENABLED_SANITIZER)
            set(ENABLED_SANITIZER "" CACHE STRING "Sanitizer to compile with" FORCE)
        endif ()
    endif ()

    if (ENABLED_SANITIZER)
        target_compile_options(${project_name} INTERFACE -fsanitize=${ENABLED_SANITIZER} -g)
        target_link_libraries(${project_name} INTERFACE -fsanitize=${ENABLED_SANITIZER} -g)
    endif ()

endfunction()
