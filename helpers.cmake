#
# Show VARIABLE = value on configuration stage
#
function(status_print var)
	message(STATUS "${var} = ${${var}}")
endfunction()

#
# Declare cache variable and print VARIABLE = value on configuration stage
#
function(declare_cache_var var default_value type docstring)
	set(${var} ${default_value} CACHE ${type} ${docstring})
	status_print(${var})
endfunction()

#
# Create target 'debug' and add function 'debug_print' which prints VARIABLE = value
# when target 'debug' is built. Useful to debug generated expressions.
#
macro(declare_target_debug debug)
    add_custom_target(${debug})
    function(debug_print var)
        add_custom_command(
            COMMAND ${CMAKE_COMMAND} -E echo ${var} = ${${var}}
            TARGET ${debug}
        )
    endfunction()
endmacro()

#
# Select default build config with support of multi config generators
#
macro(set_default_build_type config_type)
    if(CMAKE_BUILD_TYPE STREQUAL "")
        set(CMAKE_BUILD_TYPE ${config_type})
    endif()

    get_property(GENERATOR_IS_MULTI_CONFIG GLOBAL PROPERTY GENERATOR_IS_MULTI_CONFIG)
    if(GENERATOR_IS_MULTI_CONFIG)
        list(FIND CMAKE_CONFIGURATION_TYPES ${CMAKE_BUILD_TYPE} n)
        if(n LESS 0)
            message(ERROR "Configuration ${CMAKE_BUILD_TYPE} is not in CMAKE_CONFIGURATION_TYPES")
        else()
            if(CMAKE_GENERATOR STREQUAL "Ninja Multi-Config")
                set(CMAKE_DEFAULT_BUILD_TYPE ${CMAKE_BUILD_TYPE})
                status_print(CMAKE_DEFAULT_BUILD_TYPE)
            else()
                message(STATUS "Default build type is not supported for generator '${CMAKE_GENERATOR}'")
                message(STATUS "use cmake --build . --config ${config_type}")
            endif()
        endif()
    else()
        status_print(CMAKE_BUILD_TYPE)
    endif()
endmacro()