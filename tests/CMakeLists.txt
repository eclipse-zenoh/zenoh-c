message(STATUS "zenoh-c tests")

declare_cache_var(ZENOHC_BUILD_TESTS_WITH_STATIC_LIB FALSE BOOL "Use static zenohc lib for tests")

add_custom_target(tests)

file(GLOB files "${CMAKE_CURRENT_SOURCE_DIR}/*.c")
foreach(file ${files})
    get_filename_component(target ${file} NAME_WE)
    
    # Check the filename prefix to determine the test type
    if (${file} MATCHES "^.*z_api_.*$")
        set(test_type "unit")
    elseif (${file} MATCHES "^.*z_int_.*$")
        set(test_type "integration")
    else()
        message(FATAL_ERROR "Test file ${file} does not match any known type (z_api_ or z_int_)")
    endif()

    add_executable(${target} EXCLUDE_FROM_ALL ${file})
    add_dependencies(tests ${target})
    
    if(ZENOHC_BUILD_TESTS_WITH_STATIC_LIB)
        add_dependencies(${target} zenohc::static)
        target_link_libraries(${target} PRIVATE zenohc::static)
    else()
        add_dependencies(${target} zenohc::lib)
        target_link_libraries(${target} PRIVATE zenohc::lib)
        copy_dlls(${target})
    endif()

    set_property(TARGET ${target} PROPERTY C_STANDARD 11)
    # set_property(TARGET ${target} PROPERTY RUNTIME_OUTPUT_DIRECTORY "${cargo_target_dir}/tests")
    add_test(NAME "${test_type}_${target}" COMMAND ${target})
endforeach()

