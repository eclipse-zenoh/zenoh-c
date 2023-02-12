cmake_minimum_required(VERSION 3.16)
project(
	zenohc
	VERSION 0.7.0.1
	DESCRIPTION "The C bindings for Zenoh"
	HOMEPAGE_URL "https://github.com/eclipse-zenoh/zenoh-c"
	LANGUAGES C
)

include(helpers.cmake)
declare_target_projectname_debug() # create target 'zenohc_debug' to show generated expression values
check_project_usage(is_root is_ide)
status_print(is_root)
status_print(is_ide)

#
# Configuration options
#
declare_cache_var(ZENOHC_BUILD_WITH_LOGGER_AUTOINIT TRUE BOOL "Enable logger-autoinit zenoh-c feature")
declare_cache_var(ZENOHC_BUILD_IN_SOURCE_TREE ${is_ide} BOOL "Do build inside source tree")

#
# Prepare to build rust sources:
# configure Cargo.toml, copy files necessary for cargo, 
# create variables with path to cargo target directory
#
if(ZENOHC_BUILD_IN_SOURCE_TREE)
	set(cargo_toml_dir ${CMAKE_CURRENT_SOURCE_DIR})
	set(CARGO_PROJECT_DIR "") # do not put absoulte path into Cargo.toml if Cargo.toml is it's normal place
else()
	set(cargo_toml_dir ${CMAKE_CURRENT_BINARY_DIR})
	set(CARGO_PROJECT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/")
	file(COPY 
		${CARGO_PROJECT_DIR}/splitguide.yaml 
		${CARGO_PROJECT_DIR}/cbindgen.toml
		${CARGO_PROJECT_DIR}/rust-toolchain 
		DESTINATION ${cargo_toml_dir})
	set(cargo_generated_include_dir ${cargo_toml_dir}/include)
endif()
set(source_include_dir ${CMAKE_CURRENT_SOURCE_DIR}/include)
set(is_debug $<CONFIG:Debug>)
set(is_release $<NOT:${is_debug}>)
set(cargo_target_dir_debug ${cargo_toml_dir}/target/debug)
set(cargo_target_dir_release ${cargo_toml_dir}/target/release)
set(cargo_target_dir $<IF:${is_debug},${cargo_target_dir_debug},${cargo_target_dir_release}>)

debug_print(is_debug)
debug_print(cargo_target_dir)
debug_print(source_include_dir)
debug_print(cargo_generated_include_dir)

set(CARGO_PROJECT_VERSION "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")

if(NOT PROJECT_VERSION_TWEAK)
	set(CARGO_PROJECT_VERSION "${CARGO_PROJECT_VERSION}-dev")
elseif(PROJECT_VERSION_TWEAK LESS 255)
	set(CARGO_PROJECT_VERSION "${CARGO_PROJECT_VERSION}-rc.${PROJECT_VERSION_TWEAK}")
endif()

status_print(CARGO_PROJECT_VERSION)
configure_file("${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml.in" "${cargo_toml_dir}/Cargo.toml" @ONLY)

set(cargo_flags $<${is_release}:--release>)
if(ZENOHC_BUILD_WITH_LOGGER_AUTOINIT)
	set(cargo_flags ${cargo_flags} --features=logger-autoinit)
endif()

#
# Build rust sources
#
macro(set_lib list var value)
	set(${var} ${value}) 
	list(APPEND ${list} ${value})
endmacro()

# DYLIB - dymamic library (.so, .dll, .dylib)
# STATICLIB - static library (.a, .lib)
# IMPLIB - import library for windows dynamic library (DLL) - .lib
# DYLIBS - list of files required for use dynamic libraty
# STATICLIBS - list of files required for use static libraty
if(APPLE)
	set_lib(DYLIBS DYLIB "libzenohc.dylib")
	set_lib(STATICLIBS STATICLIB "libzenohc.a")
elseif(UNIX)
	set_lib(DYLIBS DYLIB "libzenohc.so")
	set_lib(STATICLIBS STATICLIB "libzenohc.a")
elseif(WIN32)
	set_lib(DYLIBS IMPLIB "zenohc.dll.lib")
	set_lib(DYLIBS DYLIB "zenohc.dll")
	set_lib(STATICLIBS STATICLIB "zenohc.lib")
endif()
status_print(DYLIBS)
status_print(STATICLIBS)
#
# add_custom_target(...BYPRODUCTS) do not support generator expressions in CMake < 3.20
# So pretend that cargo always builds both debug and release libraries
# instead of using generator expression which selects between debug/release
#
# I.e. this doesn't work in CMake < 3.20
# list(TRANSFORM DYLIBS pREPEND "${cargo_target_dir}/")
# list(TRANSFORM staticlibs PREPEND "${cargo_target_dir}/")
# ...
# BYPRODUCTS ${DYLIBS} ${STATICLIBS}
#
# so do this instead:
list(APPEND libs_debug ${DYLIBS})
list(APPEND libs_debug ${STATICLIBS})
list(APPEND libs_release ${DYLIBS})
list(APPEND libs_release ${STATICLIBS})
list(TRANSFORM libs_debug PREPEND "${cargo_target_dir_debug}/")
list(TRANSFORM libs_release PREPEND "${cargo_target_dir_release}/")
debug_print(libs_debug)
debug_print(libs_release)

set(rustflags $ENV{RUSTFLAGS})
add_custom_target(cargo ALL
	COMMAND ${CMAKE_COMMAND} -E echo \"RUSTFLAGS = ${rustflags}\"
	COMMAND ${CMAKE_COMMAND} -E echo \"cargo build ${cargo_flags}\"
	COMMAND cargo build ${cargo_flags}
	BYPRODUCTS ${libs_debug} ${libs_releasegg}
)

#
# Define libraries built by cargo as targets
#
add_library(zenohc_static STATIC IMPORTED)
add_library(zenohc SHARED IMPORTED)
add_dependencies(zenohc_static cargo)
add_dependencies(zenohc cargo)
# Workaroud for https://github.com/rust-lang/cargo/issues/5045
# mentioned in https://github.com/eclipse-zenoh/zenoh-c/issues/138
set_target_properties(zenohc PROPERTIES IMPORTED_NO_SONAME TRUE)

function(set_target_imported_locations target libname)
	set_target_properties(${target}
		PROPERTIES 
		IMPORTED_GLOBAL TRUE
		IMPORTED_LOCATION ${cargo_target_dir}/${libname}
		IMPORTED_LOCATION_DEBUG ${cargo_target_dir_debug}/${libname}
		IMPORTED_LOCATION_RELEASE ${cargo_target_dir_release}/${libname}
		IMPORTED_LOCATION_MINSIZEREL ${cargo_target_dir_release}/${libname}
		IMPORTED_LOCATION_RELWITHDEBINFO ${cargo_target_dir_release}/${libname}
	)
endfunction()

function(set_target_imported_implib target libname)
	set_target_properties(${target}
		PROPERTIES 
		IMPORTED_GLOBAL TRUE
		IMPORTED_IMPLIB ${cargo_target_dir}/${libname}
		IMPORTED_IMPLIB_DEBUG ${cargo_target_dir_debug}/${libname}
		IMPORTED_IMPLIB_RELEASE ${cargo_target_dir_release}/${libname}
		IMPORTED_IMPLIB_MINSIZEREL ${cargo_target_dir_release}/${libname}
		IMPORTED_IMPLIB_RELWITHDEBINFO ${cargo_target_dir_release}/${libname}
	)
endfunction()

set_target_imported_locations(zenohc_static ${STATICLIB})
set_target_imported_locations(zenohc ${DYLIB})
if(DEFINED zenohc_implib)
	set_target_imported_implib(zenohc ${IMPLIB})
endif()

#
# Define include directories for library targets
#
target_include_directories(zenohc_static INTERFACE ${source_include_dir})
target_include_directories(zenohc INTERFACE ${source_include_dir})
if(DEFINED cargo_generated_include_dir)
	file(MAKE_DIRECTORY ${cargo_generated_include_dir})
	target_include_directories(zenohc_static INTERFACE ${cargo_generated_include_dir})
	target_include_directories(zenohc INTERFACE ${cargo_generated_include_dir})
endif()

set_target_properties(zenohc zenohc_static PROPERTIES IMPORTED_GLOBAL TRUE)
add_library(zenohc::static ALIAS zenohc_static)
add_library(zenohc::lib ALIAS zenohc)



if(is_root)
	include(rootproject.cmake)
endif()