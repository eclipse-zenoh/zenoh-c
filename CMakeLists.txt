cmake_minimum_required(VERSION 3.20)
project(
	zenohc
	VERSION 0.7.2
	DESCRIPTION "The C bindings for Zenoh"
	HOMEPAGE_URL "https://github.com/eclipse-zenoh/zenoh-c"
	LANGUAGES C
)

include(cmake/helpers.cmake)
set_default_build_type(Release)
enable_testing()

#
# Build zenohc library from rust sources
#
# target zenohc::lib - for linking zenoh-c as dynamic library
# target zenohc::static - for linking zenoh-c as static library
#

#
# Configuration options
#
declare_cache_var_true_if_vscode(ZENOHC_BUILD_IN_SOURCE_TREE "Do build inside source tree")
declare_cache_var(ZENOHC_BUILD_WITH_LOGGER_AUTOINIT TRUE BOOL "Enable logger-autoinit zenoh-c feature")
declare_cache_var(ZENOHC_BUILD_WITH_SHARED_MEMORY TRUE BOOL "Enable shared-memory zenoh-c feature")
declare_cache_var(ZENOHC_CUSTOM_TARGET "" STRING "Rust target for cross compilation, 'aarch64-unknown-linux-gnu' for example")
declare_cache_var(ZENOHC_CARGO_CHANNEL "stable" STRING "Cargo channel selected: stable or nightly")
declare_cache_var(ZENOHC_CARGO_FLAGS "" STRING "Additional cargo flags")

#
# Prepare to build rust sources:
# configure Cargo.toml, copy files necessary for cargo,
# create variables with path to cargo target directory
#
if(ZENOHC_BUILD_IN_SOURCE_TREE AND(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_SOURCE_DIR}))
	set(cargo_toml_dir ${CMAKE_SOURCE_DIR})
	set(CARGO_PROJECT_DIR "") # do not put absoulte path into Cargo.toml if Cargo.toml is it's normal place
else()
	set(cargo_toml_dir ${CMAKE_CURRENT_BINARY_DIR})
	set(CARGO_PROJECT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/")
	file(COPY
		${CARGO_PROJECT_DIR}/splitguide.yaml
		${CARGO_PROJECT_DIR}/cbindgen.toml
		${CARGO_PROJECT_DIR}/rust-toolchain.toml
		DESTINATION ${cargo_toml_dir})
	set(cargo_generated_include_dir ${cargo_toml_dir}/include)
endif()

set(source_include_dir ${CMAKE_CURRENT_SOURCE_DIR}/include)
set(cargo_target_dir ${cargo_toml_dir}/target)
cmake_path(APPEND cargo_target_dir ${ZENOHC_CUSTOM_TARGET})
set(cargo_binary_dir_debug ${cargo_target_dir}/debug)
set(cargo_binary_dir_release ${cargo_target_dir}/release)
set(cargo_binary_dir ${cargo_target_dir}/$<IF:$<CONFIG:Debug>,debug,release>)

#
# Configure Cargo.toml
#
set(CARGO_PROJECT_VERSION "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")

if(NOT PROJECT_VERSION_TWEAK)
	set(CARGO_PROJECT_VERSION "${CARGO_PROJECT_VERSION}-dev")
elseif(PROJECT_VERSION_TWEAK LESS 255)
	set(CARGO_PROJECT_VERSION "${CARGO_PROJECT_VERSION}-rc.${PROJECT_VERSION_TWEAK}")
endif()

status_print(CARGO_PROJECT_VERSION)
configure_file("${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml.in" "${cargo_toml_dir}/Cargo.toml" @ONLY)

#
# Configure result library names
#
macro(set_lib list var value)
	set(${var} ${value})
	list(APPEND ${list} ${value})
endmacro()

# dylib - dymamic library (.so, .dll, .dylib)
# staticlib - static library (.a, .lib)
# implib - import library for windows dynamic library (DLL) - .lib
# dylibs - list of files required for use dynamic libraty
# staticlibs - list of files required for use static libraty
if(APPLE)
	set_lib(dylibs dylib "libzenohc.dylib")
	set_lib(staticlibs staticlib "libzenohc.a")
elseif(UNIX)
	set_lib(dylibs dylib "libzenohc.so")
	set_lib(staticlibs staticlib "libzenohc.a")
elseif(WIN32)
	set_lib(dylibs implib "zenohc.dll.lib")
	set_lib(dylibs dylib "zenohc.dll")
	set_lib(staticlibs staticlib "zenohc.lib")
endif()

list(APPEND libs ${dylibs})
list(APPEND libs ${staticlibs})
list(TRANSFORM libs PREPEND "${cargo_binary_dir}/")

#
# Build rust sources
#
set(cargo_flags ${ZENOHC_CARGO_FLAGS} $<$<NOT:$<CONFIG:Debug>>:--release>)

if(ZENOHC_BUILD_WITH_LOGGER_AUTOINIT)
	set(cargo_flags ${cargo_flags} --features=logger-autoinit)
endif()

if(ZENOHC_BUILD_WITH_SHARED_MEMORY)
	set(cargo_flags ${cargo_flags} --features=shared-memory)
endif()

if(NOT(ZENOHC_CUSTOM_TARGET STREQUAL ""))
	set(cargo_flags ${cargo_flags} --target=${ZENOHC_CUSTOM_TARGET})
endif()

status_print(cargo_flags)
add_custom_command(
	OUTPUT ${libs}
	COMMAND ${CMAKE_COMMAND} -E echo \"RUSTFLAGS = $$RUSTFLAGS\"
	COMMAND ${CMAKE_COMMAND} -E echo \"cargo +${ZENOHC_CARGO_CHANNEL} build ${cargo_flags}\"
	COMMAND cargo +${ZENOHC_CARGO_CHANNEL} build ${cargo_flags}
)
add_custom_target(cargo ALL DEPENDS ${libs})

#
# Define libraries built by cargo as targets
#
add_library(zenohc_static STATIC IMPORTED GLOBAL)
add_library(zenohc SHARED IMPORTED GLOBAL)
add_library(zenohc::static ALIAS zenohc_static)
add_library(zenohc::lib ALIAS zenohc)
add_dependencies(zenohc_static cargo)
add_dependencies(zenohc cargo)

# Workaroud for https://github.com/rust-lang/cargo/issues/5045
# mentioned in https://github.com/eclipse-zenoh/zenoh-c/issues/138
# If it's fixed, do not forget to correct PackageConfig.cmake.in also
set_target_properties(zenohc PROPERTIES IMPORTED_NO_SONAME TRUE)

function(set_target_imported_locations target libname)
	set_target_properties(${target}
		PROPERTIES
		IMPORTED_GLOBAL TRUE
		IMPORTED_LOCATION ${cargo_binary_dir}/${libname}
		IMPORTED_LOCATION_DEBUG ${cargo_binary_dir_debug}/${libname}
		IMPORTED_LOCATION_RELEASE ${cargo_binary_dir_release}/${libname}
		IMPORTED_LOCATION_MINSIZEREL ${cargo_binary_dir_release}/${libname}
		IMPORTED_LOCATION_RELWITHDEBINFO ${cargo_binary_dir_release}/${libname}
	)
endfunction()

function(set_target_imported_implib target libname)
	set_target_properties(${target}
		PROPERTIES
		IMPORTED_GLOBAL TRUE
		IMPORTED_IMPLIB ${cargo_binary_dir}/${libname}
		IMPORTED_IMPLIB_DEBUG ${cargo_binary_dir_debug}/${libname}
		IMPORTED_IMPLIB_RELEASE ${cargo_binary_dir_release}/${libname}
		IMPORTED_IMPLIB_MINSIZEREL ${cargo_binary_dir_release}/${libname}
		IMPORTED_IMPLIB_RELWITHDEBINFO ${cargo_binary_dir_release}/${libname}
	)
endfunction()

set_target_imported_locations(zenohc_static ${staticlib})
set_target_imported_locations(zenohc ${dylib})

if(DEFINED implib)
	set_target_imported_implib(zenohc ${implib})
endif()

# Define include directories for library targets
status_print(source_include_dir)
status_print(cargo_generated_include_dir)
target_include_directories(zenohc_static INTERFACE ${source_include_dir})
target_include_directories(zenohc INTERFACE ${source_include_dir})

if(DEFINED cargo_generated_include_dir)
	file(MAKE_DIRECTORY ${cargo_generated_include_dir})
	target_include_directories(zenohc_static INTERFACE ${cargo_generated_include_dir})
	target_include_directories(zenohc INTERFACE ${cargo_generated_include_dir})
endif()

set_target_properties(zenohc zenohc_static PROPERTIES IMPORTED_GLOBAL TRUE)

#
# Components included only if project is the root project
#
if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_SOURCE_DIR})
	include(cmake/cross_build_check.cmake)
	add_subdirectory(install)
	set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${cargo_binary_dir}/tests)
	add_subdirectory(tests)
	set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${cargo_binary_dir}/examples)
	add_subdirectory(examples)
endif()
