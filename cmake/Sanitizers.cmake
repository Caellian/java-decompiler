function(enable_sanitizers project_name)

    if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
        option(ENABLE_COVERAGE "Enable coverage reporting for gcc/clang" FALSE)

        if (ENABLE_COVERAGE)
            target_compile_options(project_options INTERFACE --coverage -O0 -g)
            target_link_libraries(project_options INTERFACE --coverage)
        endif ()

        set(SANITIZERS "")

        option(ENABLE_SANITIZER_UNDEFINED_BEHAVIOR "Enable undefined behavior sanitizer" FALSE)
        if (ENABLE_SANITIZER_UNDEFINED_BEHAVIOR)
            list(APPEND SANITIZERS "undefined")
        endif ()

        # These usually don't work together
        option(ENABLE_SANITIZER_ADDRESS "Enable address sanitizer" FALSE)
        option(ENABLE_SANITIZER_MEMORY "Enable memory sanitizer" FALSE)
        option(ENABLE_SANITIZER_THREAD "Enable thread sanitizer" FALSE)
        if (ENABLE_SANITIZER_ADDRESS)
            list(APPEND SANITIZERS "address")
        elseif (ENABLE_SANITIZER_THREAD)
            list(APPEND SANITIZERS "thread")
        elseif (ENABLE_SANITIZER_MEMORY AND CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
            list(APPEND SANITIZERS "memory")
        endif ()

        list(JOIN SANITIZERS "," LIST_OF_SANITIZERS)
    endif ()

    if (LIST_OF_SANITIZERS AND NOT "${LIST_OF_SANITIZERS}" STREQUAL "")
        target_compile_options(${project_name} INTERFACE -fsanitize=${LIST_OF_SANITIZERS})
        target_link_libraries(${project_name} INTERFACE -fsanitize=${LIST_OF_SANITIZERS})
    endif ()

endfunction()
