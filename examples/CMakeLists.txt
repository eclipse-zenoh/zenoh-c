if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_SOURCE_DIR})
    # Settings when 'examples' is the root projet
    cmake_minimum_required(VERSION 3.16)
    project(zenohc_examples LANGUAGES C)
    set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/../cmake" ${CMAKE_MODULE_PATH})
    include(helpers)
    set_default_build_type(Release)
    configure_include_project(ZENOHC zenohc zenohc::lib ".." zenohc "https://github.com/eclipse-zenoh/zenoh-c" "")
    add_custom_target(examples ALL)
else()
    message(STATUS "zenoh-c examples")
    add_custom_target(examples)
endif()

file(GLOB files "${CMAKE_CURRENT_SOURCE_DIR}/*.c")

foreach(file ${files})
    get_filename_component(target ${file} NAME_WE)

    # FIXME: remove once zenoh time primitives are available and examples updated
    if(NOT(UNIX) AND(${target} STREQUAL "z_ping" OR ${target} STREQUAL "z_pong" OR ${target} STREQUAL "z_sub_thr"))
        continue()
    endif()

    add_executable(${target} EXCLUDE_FROM_ALL ${file})
    add_dependencies(examples ${target})

    add_dependencies(${target} zenohc::lib)
    target_link_libraries(${target} PRIVATE zenohc::lib)
    copy_dlls(${target})

    set_property(TARGET ${target} PROPERTY C_STANDARD 11)
endforeach()
