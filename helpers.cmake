include_guard()

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
# Declare cache variable which is set to TRUE if project is supposedly
# loaded as root project into vscode
#
function(declare_cache_var_true_if_vscode var docstring)
    if(CMAKE_CURRENT_BINARY_DIR STREQUAL "${CMAKE_CURRENT_SOURCE_DIR}/build")
        set(in_vscode TRUE)
    endif()
    declare_cache_var(${var} ${in_vscode} BOOL ${docstring})
endfunction()

#
# Create target named '${PROJECT_NAME}_debug' and add function 'debug_print' which prints VARIABLE = value
# when this target is built. Useful to debug generated expressions.
#`
macro(declare_target_projectname_debug)
    add_custom_target(${PROJECT_NAME}_debug)
    function(debug_print var)
        add_custom_command(
            COMMAND ${CMAKE_COMMAND} -E echo ${var} = ${${var}}
            TARGET ${PROJECT_NAME}_debug
        )
    endfunction()
endmacro()

#
# Select default build config with support of multi config generators
#
macro(set_default_build_type config_type)
    get_property(GENERATOR_IS_MULTI_CONFIG GLOBAL PROPERTY GENERATOR_IS_MULTI_CONFIG)
    if(GENERATOR_IS_MULTI_CONFIG)
        if(NOT DEFINED CMAKE_BUILD_TYPE) # if user passed argument '-DCMAKE_BUILD_TYPE=value', use it
            set(CMAKE_BUILD_TYPE ${config_type})
        endif()
         list(FIND CMAKE_CONFIGURATION_TYPES ${CMAKE_BUILD_TYPE} n)
        if(n LESS 0)
            message(FATAL_ERROR "Configuration ${CMAKE_BUILD_TYPE} is not in CMAKE_CONFIGURATION_TYPES")
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
        if(CMAKE_BUILD_TYPE STREQUAL "")
            set(CMAKE_BUILD_TYPE ${config_type})
        endif()
         status_print(CMAKE_BUILD_TYPE)
    endif()
endmacro()


#
# If target property is set, get it to var
# (necessary to avoid CMake behavior - setting variable to <VAR>-NOTFOUND)
#
function(get_target_property_if_set var target property)
    unset(${${var}})
    get_property(is_set TARGET ${target} PROPERTY ${property} SET)
    if (is_set)
        get_property(value TARGET ${target} PROPERTY ${property})
        set(${var} ${value} PARENT_SCOPE)
    endif()
endfunction()

#
# If <var> is not empty, join string <s> to filename part of it
#
function(join_string_to_filename var s)
    if((${${var}} STREQUAL "") OR (${s} STREQUAL ""))
        return()
    endif()
    get_filename_component(dir "${${var}}" DIRECTORY)
    get_filename_component(name_we "${${var}}" NAME_WE)
    get_filename_component(ext "${${var}}" EXT)
    set(${${var}} ${dir}${name_we}${s}${ext} PARENT_SCOPE)
endfunction()



# and rename file to new name. This is necessary to be able to install file its new name. 
# The 'install' operation have 'RENAME' option, but generator
# expressions are not supported there in CMake < 3.20
