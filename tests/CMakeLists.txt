message(STATUS "zenoh-c tests")

add_custom_target(tests)

find_package(Threads REQUIRED)

file(GLOB files "${CMAKE_CURRENT_SOURCE_DIR}/*.c")
foreach(file ${files})
    get_filename_component(target ${file} NAME_WE)
    
    # Exclude SHM tests if SHM feature is disabled
    if(NOT(ZENOHC_BUILD_WITH_SHARED_MEMORY))
	    if(${target} MATCHES "^.*_shm.*$")
            continue()
        endif()
    endif()

    # Exclude Liveliness and zenoh-ext tests if unstable api feature is disabled
    if(NOT(ZENOHC_BUILD_WITH_UNSTABLE_API))
	    if((${target} MATCHES "^.*_liveliness.*$") OR (${target} MATCHES "^.*_query_sub_test.*$"))
            continue()
        endif()
    endif()

    # Check the filename prefix to determine the test type
    if (${file} MATCHES "^.*z_api_.*$")
        set(test_type "unit")
    elseif (${file} MATCHES "^.*z_int_.*$")
        set(test_type "integration")
    elseif (${file} MATCHES "^.*z_build_.*$")
        set(test_type "build")
    elseif (${file} MATCHES "^.*z_leak_.*$")
        set(test_type "leak")
    else()
        message(FATAL_ERROR "Test file ${file} does not match any known type (z_api_ or z_int_ or z_build)")
    endif()

    add_executable(${target} EXCLUDE_FROM_ALL ${file})
    add_dependencies(tests ${target})
    add_dependencies(${target} zenohc::lib)
    target_link_libraries(${target} PRIVATE zenohc::lib Threads::Threads)
    copy_dlls(${target})
    set_property(TARGET ${target} PROPERTY C_STANDARD 11)
    if (NOT(test_type STREQUAL ${CMAKE_CURRENT_SOURCE_DIR}))
        add_test(NAME "${test_type}_${target}" COMMAND ${target})
    endif()
endforeach()

