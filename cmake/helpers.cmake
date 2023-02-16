include_guard()

#
# Show VARIABLE = value on configuration stage
#
macro(status_print var)
	message(STATUS "${var} = ${${var}}")
endmacro()

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
    else()
        set(in_vscode FALSE)
    endif()
    declare_cache_var(${var} ${in_vscode} BOOL ${docstring})
endfunction()

#
# Create target named 'debug_print' which prints VARIABLE = value
# when this target is built. Useful to debug generated expressions.
#`
function(debug_print var)
    if(NOT TARGET debug_print)
        add_custom_target(debug_print GLOBAL)
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

#
# Unset variables if they have empty string value
#
macro(unset_if_empty vars)
    foreach(var ${vars})
        if(${var} STREQUAL "")
            unset(${var})
        endif()
    endforeach()
endmacro()

#
# Usage:
#
# include_project(<project_name> TARGET <target> 
#  [PATH <project_path>] |
#  [PACKAGE <package_name>] | 
#  [GIT_URL <git_url> [GIT_TAG <git_tag>]]
# )
#
# includes CMake project with one of the following ways:
#   add_subdirectory(project_path) or
#   find_package(package_name) or
#   FetchContent(git_url)
#
# Example:
# include_project(zenohc TARGET zenohc::lib PATH "${CMAKE_CURRENT_SOURCE_DIR}..\zenoh_c" )
#
function(include_project project_name)
    cmake_parse_arguments(PARSE_ARGV 1 "ARG" "" "TARGET;PATH;PACKAGE;GIT_URL;GIT_TAG" "")
    unset_if_empty(ARG_PATH ARG_TARGET ARG_PACKAGE ARG_GIT_URL)
    if(NOT DEFINED ARG_TARGET)
        message(FATAL_ERROR "Non-empty TARGET parameter is required")
    endif()
    if(TARGET ${ARG_TARGET})
        message(FATAL_ERROR "Target '${ARG_TARGET}' already defined")
    endif()

    if(DEFINED ARG_PATH)
        message(STATUS "include project '${project_name} from directory '${ARG_PATH}'")
        list(APPEND CMAKE_MESSAGE_INDENT "  ")
        add_subdirectory(${ARG_PATH} ${project_name})
        list(POP_BACK CMAKE_MESSAGE_INDENT)
        if(TARGET ${ARG_TARGET})
            return()
        endif()
        message(FATAL_ERROR "Project at '${ARG_PATH}' should define target ${ARG_TARGET}")
    elseif(DEFINED ARG_PACKAGE)
        # Give priority to install directory
        # Useful for development when older version of the project version may be installed in system
        #
        # TODO: "if( NOT TARGET" below should be not necessary 
        # (see https://cmake.org/cmake/help/latest/command/find_package.html, search for "override the order")
        # but in fact cmake fails without it when zenohc is present both in CMAKE_INSTALL_PREFIX and in /usr/local.
        # Consider is it still necessary after next bumping up cmake version
        find_package(${ARG_PACKAGE} PATHS ${CMAKE_INSTALL_PREFIX} NO_DEFAULT_PATH QUIET)
        if(NOT TARGET ${ARG_TARGET})
            find_package(${ARG_PACKAGE} QUIET)
        endif()
        set(package_path ${${ARG_PACKAGE}_CONFIG})
        if(TARGET ${ARG_TARGET})
            message(STATUS "included project '${project_name}' from package '${ARG_PACKAGE}' on path '${package_path}'")
            return()
        endif()
        if("${package_path}" STREQUAL "")
            message(FATAL_ERROR "Package '${ARG_PACKAGE}' not found")
        else()
            message(FATAL_ERROR "Package '${ARG_PACKAGE}' on path '${package_path}' doesn't define target '${ARG_TARGET}")
        endif()
    elseif(DEFINED ARG_GIT_URL)
        if(DEFINED ARG_GIT_TAG)
            set(git_url "${ARG_GIT_URL}#{ARG_GIT_TAG}")
        else()
            set(git_url ${ARG_GIT_URL})
        endif()
        message(STATUS "including project '${project_name} from git '${git_url}'")
        list(APPEND CMAKE_MESSAGE_INDENT "  ")
        if(DEFINED ARG_GIT_TAG)
            FetchContent_Declare(${project_name}
                GIT_REPOSITORY ${ARG_GIT_URL}
                GIT_TAG ${ARG_GIT_TAG}
            )
        else()
            FetchContent_Declare(${project_name}
                GIT_REPOSITORY ${ARG_GIT_URL}
            )
        endif()
        FetchContent_MakeAvailable(${project_name})
        list(POP_BACK CMAKE_MESSAGE_INDENT)
        if(TARGET ${ARG_TARGET})
            return()
        endif()
        message(FATAL_ERROR "Project at ${git_url} should define target ${ARG_TARGET}")
    else()
        message(FATAL_ERROR "No source for project '${project_name}' specified")
    endif()

endfunction()
