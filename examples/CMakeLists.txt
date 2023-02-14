message(STATUS "zenoh-c examples")

declare_cache_var(ZENOHC_BUILD_EXAMPLES_WITH_STATIC_LIB FALSE BOOL "Use static zenohc lib for examples")

add_custom_target(examples)

file(GLOB files "${CMAKE_CURRENT_SOURCE_DIR}/*.c")
foreach(file ${files})
    get_filename_component(target ${file} NAME_WE)
    add_executable(${target} EXCLUDE_FROM_ALL ${file})
    add_dependencies(examples ${target})
    add_platfrom_libraries(${target})
    if(ZENOHC_BUILD_EXAMPLES_WITH_STATIC_LIB)
        add_dependencies(${target} zenohc::static)
    else()
        add_dependencies(${target} zenohc::lib)
        copy_dlls(${target})
    endif()
    set_property(TARGET ${target} PROPERTY C_STANDARD 11)
    # set_property(TARGET ${target} PROPERTY RUNTIME_OUTPUT_DIRECTORY "${cargo_target_dir}/examples")
endforeach()

