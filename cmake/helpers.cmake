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
# Create target named 'debug_print' which prints VARIABLE = value
# when this target is built. Useful to debug generated expressions.
#`
function(debug_print var)
    if(NOT TARGET debug_print)
        add_custom_target(${target} GLOBAL)
    endif()
    add_custom_command(
        COMMAND ${CMAKE_COMMAND} -E echo ${var} = ${${var}}
        TARGET debug_print
    )
endfunction()

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
# Add default set of libraries depending on platform
#
function(add_platfrom_libraries target)
	if(APPLE)
		find_library(FFoundation Foundation)
		find_library(FSecurity Security)
		target_link_libraries(${target} PUBLIC ${FFoundation} ${FSecurity})
	elseif(UNIX)
		target_link_libraries(${target} PUBLIC rt pthread m dl)
	elseif(WIN32)
		target_link_libraries(${target} PUBLIC ws2_32 crypt32 secur32 bcrypt ncrypt userenv ntdll iphlpapi runtimeobject)
	endif()

endfunction()

#
# Copy mecessary dlls to target runtime directory
#
function(copy_dlls target)
	if(WIN32)
		add_custom_command(TARGET ${target} POST_BUILD
			COMMAND ${CMAKE_COMMAND} -E copy_if_different $<TARGET_RUNTIME_DLLS:${target}> $<TARGET_FILE_DIR:${target}>
			COMMAND_EXPAND_LISTS
		)
	endif()   
endfunction()

# 
# get property value avoiding CMake behavior - setting variable to <VAR>-NOTFOUND for undefined property
#
function(get_target_property_if_set var target property)
    get_property(is_set TARGET ${target} PROPERTY ${property} SET)
    if (NOT is_set)
		unset(${var} PARENT_SCOPE)
		return()
	endif()
	get_property(value TARGET ${target} PROPERTY ${property})
	set(${var} ${value} PARENT_SCOPE)
endfunction()

