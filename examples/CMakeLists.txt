if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_SOURCE_DIR})
    # Settings when 'examples' is the root projet
    cmake_minimum_required(VERSION 3.20)
    project(zenohc_examples LANGUAGES C)
    include(../cmake/helpers.cmake)
    set_default_build_type(Release)
    configure_include_project(ZENOHC zenohc zenohc::lib ".." zenohc "https://github.com/eclipse-zenoh/zenoh-c" "")
    add_custom_target(examples ALL)
else()
    message(STATUS "zenoh-c examples")
    add_custom_target(examples)
endif()

declare_cache_var(ZENOHC_BUILD_EXAMPLES_WITH_STATIC_LIB FALSE BOOL "Use static zenohc lib for examples")

file(GLOB files "${CMAKE_CURRENT_SOURCE_DIR}/*.c")

foreach(file ${files})
    get_filename_component(target ${file} NAME_WE)

    if(NOT(UNIX) AND(${target} STREQUAL "z_ping" OR ${target} STREQUAL "z_pong"))
        continue()
    endif()

    add_executable(${target} EXCLUDE_FROM_ALL ${file})
    add_dependencies(examples ${target})

    if(ZENOHC_BUILD_EXAMPLES_WITH_STATIC_LIB)
        add_dependencies(${target} zenohc::static)
        target_link_libraries(${target} PRIVATE zenohc::static)
    else()
        add_dependencies(${target} zenohc::lib)
        target_link_libraries(${target} PRIVATE zenohc::lib)
        copy_dlls(${target})
    endif()

    set_property(TARGET ${target} PROPERTY C_STANDARD 11)
endforeach()
