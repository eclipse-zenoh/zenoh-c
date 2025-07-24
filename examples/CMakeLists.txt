if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_SOURCE_DIR})
    # Settings when 'examples' is the root projet
    cmake_minimum_required(VERSION 3.16)
    project(zenohc_examples LANGUAGES C)
    set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/../cmake" ${CMAKE_MODULE_PATH})
    include(helpers)
    find_package(zenohc REQUIRED)
    add_custom_target(examples ALL)
else()
    message(STATUS "zenoh-c examples")
    add_custom_target(examples)
endif()

file(GLOB files "${CMAKE_CURRENT_SOURCE_DIR}/*.c")

foreach(file ${files})
    get_filename_component(target ${file} NAME_WE)
    # Exclude SHM examples if SHM feature is disabled
    if(NOT(ZENOHC_BUILD_WITH_SHARED_MEMORY AND (ZENOHC_BUILD_WITH_UNSTABLE_API)))
        if(${target} MATCHES "^.*_shm.*$")
            continue()
        endif()
    endif()
    # Exclude Liveliness and zenoh-ext examples if unstable api feature is disabled
    if(NOT(ZENOHC_BUILD_WITH_UNSTABLE_API))
        if(
            (${target} MATCHES "^.*_advanced_sub.*$")
            OR (${target} MATCHES "^.*_advanced_pub.*$") 
        )
            continue()
        endif()
    endif()

    add_executable(${target} EXCLUDE_FROM_ALL ${file})
    add_dependencies(examples ${target})

    add_dependencies(${target} zenohc::lib)
    target_link_libraries(${target} PRIVATE zenohc::lib)
    copy_dlls(${target})

    set_property(TARGET ${target} PROPERTY C_STANDARD 11)
endforeach()
