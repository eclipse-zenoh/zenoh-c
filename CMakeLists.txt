cmake_minimum_required(VERSION 3.16)

file(READ ${CMAKE_CURRENT_SOURCE_DIR}/version.txt version)

project(
	zenohc
	VERSION ${version}
	DESCRIPTION "The C bindings for Zenoh"
	HOMEPAGE_URL "https://github.com/eclipse-zenoh/zenoh-c"
	LANGUAGES C CXX
)

set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake" ${CMAKE_MODULE_PATH})
include(helpers)
set_default_build_type(Release)
enable_testing()

#
# Build zenohc library from rust sources
#
# target zenohc::lib - for linking zenoh-c as static or dynamic library depending on BUILD_SHARED_LIBS boolean cache var
# target zenohc::static - for linking zenoh-c as static library
# target zenohc::shared - for linking zenoh-c as dynamic library
#

#
# Configuration options
#
declare_cache_var_true_if_vscode(ZENOHC_BUILD_IN_SOURCE_TREE "Do build inside source tree")
declare_cache_var(ZENOHC_BUILD_WITH_SHARED_MEMORY FALSE BOOL "Enable shared-memory zenoh-c feature")
declare_cache_var(ZENOHC_BUILD_WITH_UNSTABLE_API FALSE BOOL "Enable unstable API feature")
declare_cache_var(ZENOHC_BUILD_TESTS_WITH_CXX FALSE BOOL "Use C++ compiler for building tests to check API's C++ compatibility")
declare_cache_var(ZENOHC_CUSTOM_TARGET "" STRING "Rust target for cross compilation, 'aarch64-unknown-linux-gnu' for example")
message(STATUS "ENV{CARGO_BUILD_TARGET} = $ENV{CARGO_BUILD_TARGET}")
if(NOT(ZENOHC_CUSTOM_TARGET STREQUAL ""))
	set(_INTERNAL_ZENOHC_BUILD_TARGET ${ZENOHC_CUSTOM_TARGET})
	set(_INTERNAL_ZENOHC_BUILD_TARGET_SOURCE ZENOHC_CUSTOM_TARGET)
elseif(NOT("$ENV{CARGO_BUILD_TARGET}" STREQUAL ""))
	set(_INTERNAL_ZENOHC_BUILD_TARGET $ENV{CARGO_BUILD_TARGET})
	set(_INTERNAL_ZENOHC_BUILD_TARGET_SOURCE ENV{CARGO_BUILD_TARGET})
endif()
if(NOT("${_INTERNAL_ZENOHC_BUILD_TARGET}" STREQUAL ""))
	message(STATUS "Build target: ${_INTERNAL_ZENOHC_BUILD_TARGET} (from ${_INTERNAL_ZENOHC_BUILD_TARGET_SOURCE})")
endif()
declare_cache_var(ZENOHC_CARGO_CHANNEL "" STRING "Cargo channel parameter. Should be '+stable', '+nightly' or empty value")
declare_cache_var(ZENOHC_CARGO_FLAGS "" STRING "Additional cargo flags")
declare_cache_var(BUILD_SHARED_LIBS TRUE BOOL "Alias zenohc::lib target to zenohc::shared if TRUE, to zenohc::static if FALSE")
declare_cache_var(ZENOHC_TREAT_WARNING_AS_ERROR OFF BOOL "Whether to treat compilation warnings as errors")
set(CMAKE_COMPILE_WARNING_AS_ERROR ${ZENOHC_TREAT_WARNING_AS_ERROR})

set(project_version "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")
if(PROJECT_VERSION_TWEAK STREQUAL "")
	set(project_version "${project_version}")
elseif(PROJECT_VERSION_TWEAK EQUAL 0)
	set(project_version "${project_version}-dev")
elseif(PROJECT_VERSION_TWEAK GREATER 0)
	set(project_version "${project_version}-pre.${PROJECT_VERSION_TWEAK}")
endif()
status_print(project_version)

#
# There are 3 possible variants of placement generated Cargo.toml files:
# 1. Build in source tree (in IDE usually), using single config generator (Ninja, Makefiles)
#
#    In this case Cargo.toml is placed at the root of source tree to make it visible for rust-analyzer. When release or debug
#    configuration is selected, Cargo.toml is updated accordingly
#
# 2. Build in source tree (in IDE usually), using multi config generator (Visual Studio, Ninja Multi-Config)
#
#    Cargo.toml is placed at the root of source tree to make it visible for rust-analyzer. Also two additional Cargo.toml files
#    are placed in ${CMAKE_CURRENT_BINARY_DIR}/debug and ${CMAKE_CURRENT_BINARY_DIR}/release directories configured for debug and
#    release builds respectively
#
# 3. Build in build tree, no matter what generator is used
#
#    Cargo.toml is placed in ${CMAKE_CURRENT_BINARY_DIR}/debug and ${CMAKE_CURRENT_BINARY_DIR}/release directories. No care is taken
#    about Cargo.toml at the root of source tree
#
if(ZENOHC_BUILD_IN_SOURCE_TREE AND(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_SOURCE_DIR}))
	set(cargo_toml_dir_ide ${CMAKE_SOURCE_DIR})
	if (GENERATOR_IS_MULTI_CONFIG)
		message(STATUS "Mode: IDE, Multi-Config generator (${CMAKE_GENERATOR}))")
		set(cargo_toml_dir_debug ${CMAKE_CURRENT_BINARY_DIR}/debug)
		set(cargo_toml_dir_release ${CMAKE_CURRENT_BINARY_DIR}/release)
		file(MAKE_DIRECTORY ${cargo_toml_dir_debug}/include)
		file(MAKE_DIRECTORY ${cargo_toml_dir_release}/include)
	else()
		message(STATUS "Mode: IDE, Single-Config generator (${CMAKE_GENERATOR})")
		set(cargo_toml_dir_debug ${cargo_toml_dir_ide})
		set(cargo_toml_dir_release ${cargo_toml_dir_ide})
	endif()
else()
	message(STATUS "Mode: Non-IDE")
	unset(cargo_toml_dir_ide)
	set(cargo_toml_dir_debug ${CMAKE_CURRENT_BINARY_DIR}/debug)
	set(cargo_toml_dir_release ${CMAKE_CURRENT_BINARY_DIR}/release)
	file(MAKE_DIRECTORY ${cargo_toml_dir_debug}/include)
	file(MAKE_DIRECTORY ${cargo_toml_dir_release}/include)
endif()

# note: case-insensitive build configuration check
string(TOLOWER "${CMAKE_BUILD_TYPE}" CMAKE_BUILD_TYPE_LOWER)
if (CMAKE_BUILD_TYPE_LOWER STREQUAL "debug")
	set(DEBUG TRUE)
endif()

set_genexpr_condition(cargo_toml_dir DEBUG $<CONFIG:Debug> ${cargo_toml_dir_debug} ${cargo_toml_dir_release})
set_genexpr_condition(cargo_generated_include_dir DEBUG $<CONFIG:Debug> ${cargo_toml_dir_debug}/include ${cargo_toml_dir_release}/include)

set(cargo_target_dir_debug ${cargo_toml_dir_debug}/target)
set(cargo_target_dir_release ${cargo_toml_dir_release}/target)
if(NOT("${_INTERNAL_ZENOHC_BUILD_TARGET}" STREQUAL ""))
	set(cargo_target_dir_debug ${cargo_target_dir_debug}/${_INTERNAL_ZENOHC_BUILD_TARGET})
	set(cargo_target_dir_release ${cargo_target_dir_release}/${_INTERNAL_ZENOHC_BUILD_TARGET})
endif()
set(cargo_binary_dir_debug ${cargo_target_dir_debug}/debug)
set(cargo_binary_dir_release ${cargo_target_dir_release}/release)
set_genexpr_condition(cargo_binary_dir DEBUG $<CONFIG:Debug> ${cargo_binary_dir_debug} ${cargo_binary_dir_release})
set(source_include_dir ${CMAKE_CURRENT_SOURCE_DIR}/include)

function(configure_cargo_toml cargo_toml_dir CARGO_PROJECT_VERSION CARGO_LIB_NAME)
	message(STATUS "Configuring Cargo.toml in ${cargo_toml_dir} for ${CARGO_LIB_NAME}")
	if(NOT(cargo_toml_dir STREQUAL ${CMAKE_CURRENT_SOURCE_DIR}))
		set(CARGO_PROJECT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/")
		file(COPY
			${CMAKE_CURRENT_SOURCE_DIR}/splitguide.yaml
			${CMAKE_CURRENT_SOURCE_DIR}/cbindgen.toml
			${CMAKE_CURRENT_SOURCE_DIR}/rust-toolchain.toml
			${CMAKE_CURRENT_SOURCE_DIR}/version.txt
			DESTINATION ${cargo_toml_dir})
		if(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.lock")
			file(COPY
				${CMAKE_CURRENT_SOURCE_DIR}/Cargo.lock
				DESTINATION ${cargo_toml_dir})
		endif()
		file(COPY
			${CMAKE_CURRENT_SOURCE_DIR}/include/zenoh.h
			${CMAKE_CURRENT_SOURCE_DIR}/include/zenoh_memory.h
			${CMAKE_CURRENT_SOURCE_DIR}/include/zenoh_constants.h
			DESTINATION ${cargo_toml_dir}/include/)
	endif()
	configure_file("${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml.in" "${cargo_toml_dir}/Cargo.toml" @ONLY)
endfunction()

#
# Configure Cargo.toml files
#
# Set COMMENT_PINNED_DEPS based on ZENOHC_CARGO_CHANNEL
if(ZENOHC_CARGO_CHANNEL STREQUAL "+1.75" OR ZENOHC_CARGO_CHANNEL STREQUAL "+1.75.0")
	set(COMMENT_PINNED_DEPS "")
else()
	set(COMMENT_PINNED_DEPS "# ")
endif()

set(cargo_lib_name_debug zenohc)
set(cargo_lib_name_release zenohc)
if(cargo_toml_dir_debug STREQUAL cargo_toml_dir_release)
	# same Cargo.toml is for ide, debug and release configurations
	# This happens only for non-multiconfig generators, so testing for debug/release on configuration stage is allowed
	# note: case-insensitive build configuration check
	string(TOLOWER "${CMAKE_BUILD_TYPE}" CMAKE_BUILD_TYPE_LOWER)
	if (CMAKE_BUILD_TYPE_LOWER STREQUAL "debug")
		set(cargo_lib_name ${cargo_lib_name_debug})
	else()
		set(cargo_lib_name ${cargo_lib_name_release})
	endif()
	configure_cargo_toml(${cargo_toml_dir_ide} ${project_version} ${cargo_lib_name})
else()
	set_genexpr_condition(cargo_lib_name DEBUG $<CONFIG:Debug> ${cargo_lib_name_debug} ${cargo_lib_name_release})
	if(DEFINED cargo_toml_dir_ide)
		configure_cargo_toml(${cargo_toml_dir_ide} ${project_version} ${cargo_lib_name_release})
	endif()
	configure_cargo_toml(${cargo_toml_dir_debug} ${project_version} ${cargo_lib_name_debug})
	configure_cargo_toml(${cargo_toml_dir_release} ${project_version} ${cargo_lib_name_release})
endif()

#
# Copy toolchain configuration to build directory to ensure it is used when cargo is invoked from a directory other
# than the directory containing the manifest file
#
if(NOT EXISTS "${CMAKE_CURRENT_BINARY_DIR}/rust-toolchain.toml")
	file(COPY
		${CMAKE_CURRENT_SOURCE_DIR}/rust-toolchain.toml
		DESTINATION ${CMAKE_CURRENT_BINARY_DIR})
endif()

#
# Configure result library names
#
macro(set_lib list var value)
	set(${var} ${value})
	list(APPEND ${list} ${value})
endmacro()

# dylib[r|d] - dymamic library (.so, .dll, .dylib)
# staticlib[r|d] - static library (.a, .lib)
# implib[r|d] - import library for windows dynamic library (DLL) - .lib
# dylibs[r|d] - list of files required for use dynamic library
# staticlibs[r|d] - list of files required for use static library
# get rust output library names from https://github.com/corrosion-rs/corrosion/blob/1c6974c2473765449e7c4649f9f96f1b751420c3/cmake/Corrosion.cmake#L331
if(WIN32)
	set_lib(dylibsr dylibr zenohc.dll)
	set_lib(dylibsd dylibd zenohc.dll)
	if(MSVC)
		set_lib(staticlibsr staticlibr zenohc.lib)
		set_lib(staticlibsd staticlibd zenohc.lib)
		set_lib(dylibsr implibr zenohc.dll.lib)
		set_lib(dylibsd implibd zenohc.dll.lib)
	else() #gnu/mingw/msys ?
		set_lib(staticlibsr staticlibr libzenohc.a)
		set_lib(staticlibsd staticlibd libzenohc.a)
		set_lib(dylibsr implibr libzenohc.dll.a)
		set_lib(dylibsd implibd libzenohc.dll.a)
	endif()
elseif(APPLE)
	set_lib(dylibsr dylibr libzenohc.dylib)
	set_lib(dylibsd dylibd libzenohc.dylib)
	set_lib(staticlibsr staticlibr libzenohc.a)
	set_lib(staticlibsd staticlibd libzenohc.a)
else() #UNIX
	set_lib(dylibsr dylibr libzenohc.so)
	set_lib(dylibsd dylibd libzenohc.so)
	set_lib(staticlibsr staticlibr libzenohc.a)
	set_lib(staticlibsd staticlibd libzenohc.a)
endif()

list(APPEND libsr ${dylibsr})
list(APPEND libsr ${staticlibsr})
list(APPEND libsd ${dylibsd})
list(APPEND libsd ${staticlibsd})
list(TRANSFORM libsr PREPEND "${cargo_binary_dir_release}/")
list(TRANSFORM libsd PREPEND "${cargo_binary_dir_debug}/")
set_genexpr_condition(libs DEBUG $<CONFIG:Debug> "${libsd}" "${libsr}")

#
# Build rust sources
#

# Combine "--release" and "--manifest-path" options under DEBUG condition to avoid passing empty parameter to cargo command line in `add_custom_command`, causing build failure
# This empty item ($<IF:$<CONFIG:Debug>;,--release>) can't be filtered out by `list(FILTER ...)` because it becomes empty only on
# build stage when generator expressions are evaluated.
set_genexpr_condition(cargo_flags DEBUG $<CONFIG:Debug>
	"--manifest-path=${cargo_toml_dir_debug}/Cargo.toml"
	"--release;--manifest-path=${cargo_toml_dir_release}/Cargo.toml")
set(cargo_flags ${cargo_flags} ${ZENOHC_CARGO_FLAGS})

if(ZENOHC_BUILD_WITH_SHARED_MEMORY)
	set(cargo_flags ${cargo_flags} --features=shared-memory)
elseif("${cargo_flags}" MATCHES ^.*shared-memory.*$)
	set(ZENOHC_BUILD_WITH_SHARED_MEMORY TRUE)
	message(STATUS "Due to ZENOHC_CARGO_FLAGS setting ZENOHC_BUILD_WITH_SHARED_MEMORY = TRUE")
endif()

if(ZENOHC_BUILD_WITH_UNSTABLE_API)
	set(cargo_flags ${cargo_flags} --features=unstable)
elseif("${cargo_flags}" MATCHES ^.*unstable.*$)
	set(ZENOHC_BUILD_WITH_UNSTABLE_API TRUE)
	message(STATUS "Due to ZENOHC_CARGO_FLAGS setting ZENOHC_BUILD_WITH_UNSTABLE_API = TRUE")
endif()


if(NOT(ZENOHC_CUSTOM_TARGET STREQUAL ""))
	set(cargo_flags ${cargo_flags} --target=${ZENOHC_CUSTOM_TARGET})
endif()

status_print(cargo_flags)
status_print(libs)
file(GLOB_RECURSE rust_sources "Cargo.toml.in" "src/*.rs" "build.rs" "splitguide.yaml")
add_custom_command(
	OUTPUT ${libs}
	COMMAND ${CMAKE_COMMAND} -E echo \"RUSTFLAGS = $$RUSTFLAGS\"
	COMMAND ${CMAKE_COMMAND} -E echo \"cargo ${ZENOHC_CARGO_CHANNEL} build ${cargo_flags}\"
	COMMAND ${CMAKE_COMMAND} -E env OPAQUE_TYPES_BUILD_DIR=${CMAKE_BINARY_DIR}/opaque-types cargo ${ZENOHC_CARGO_CHANNEL} build ${cargo_flags}
	VERBATIM
	COMMAND_EXPAND_LISTS
	DEPENDS "${rust_sources}"
	USES_TERMINAL
)
add_custom_target(cargo ALL DEPENDS "${libs}")

#
# Define libraries built by cargo as targets
#

function(set_target_imported_locations target libr libd)
	set(_loc_debug  ${cargo_binary_dir_debug}/${libd})
	set(_loc_release ${cargo_binary_dir_release}/${libr})
	if(CMAKE_BUILD_TYPE STREQUAL "Debug")
		set(_loc "${_loc_debug}")
	else()
		set(_loc "${_loc_release}")
	endif()
	set_target_properties(${target}
		PROPERTIES
		IMPORTED_GLOBAL TRUE
		IMPORTED_LOCATION ${_loc}
		IMPORTED_LOCATION_DEBUG ${_loc_debug}
		IMPORTED_LOCATION_RELEASE ${_loc_release}
		IMPORTED_LOCATION_MINSIZEREL ${_loc_release}
		IMPORTED_LOCATION_RELWITHDEBINFO ${_loc_release}
	)
endfunction()

function(set_target_imported_implib target libr libd)
	set(_loc_debug  ${cargo_binary_dir_debug}/${libd})
	set(_loc_release ${cargo_binary_dir_release}/${libr})
	if(CMAKE_BUILD_TYPE STREQUAL "Debug")
		set(_loc "${_loc_debug}")
	else()
		set(_loc "${_loc_release}")
	endif()
	set_target_properties(${target}
		PROPERTIES
		IMPORTED_GLOBAL TRUE
		IMPORTED_IMPLIB ${_loc}
		IMPORTED_IMPLIB_DEBUG ${_loc_debug}
		IMPORTED_IMPLIB_RELEASE ${_loc_release}
		IMPORTED_IMPLIB_MINSIZEREL ${_loc_release}
		IMPORTED_IMPLIB_RELWITHDEBINFO ${_loc_release}
	)
endfunction()

#
# Setup additional properties for library targets
# *IMPORTANT* any options in this section should be repeated in install/PackageConfig.cmake.in
# to achieve correct behavior of find_package(zenohc)
#
add_library(zenohc_shared SHARED IMPORTED GLOBAL)
add_library(zenohc::shared ALIAS zenohc_shared)
add_dependencies(zenohc_shared cargo)
target_compile_definitions(zenohc_shared INTERFACE ZENOHC_DYN_LIB)
set_target_imported_locations(zenohc_shared ${dylibr} ${dylibd})
add_library(zenohc_static STATIC IMPORTED GLOBAL)
add_library(zenohc::static ALIAS zenohc_static)
add_dependencies(zenohc_static cargo)
get_required_static_libs(NATIVE_STATIC_LIBS)
target_link_libraries(zenohc_static INTERFACE ${NATIVE_STATIC_LIBS})
set_target_imported_locations(zenohc_static ${staticlibr} ${staticlibd})
target_include_directories(zenohc_static INTERFACE ${cargo_generated_include_dir})
set_target_properties(zenohc_static PROPERTIES IMPORTED_GLOBAL TRUE)

if(DEFINED implibr)
	set_target_imported_implib(zenohc_shared ${implibr} ${implibd})
endif()
target_include_directories(zenohc_shared INTERFACE ${cargo_generated_include_dir})
set_target_properties(zenohc_shared PROPERTIES IMPORTED_GLOBAL TRUE)
if (BUILD_SHARED_LIBS)
	add_library(zenohc::lib ALIAS zenohc_shared)
else()
	add_library(zenohc::lib ALIAS zenohc_static)
endif()


#
# Components included only if project is the root project
#
if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_SOURCE_DIR})
	add_subdirectory(install)
	set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${cargo_binary_dir}/tests)
	add_subdirectory(tests)
	set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${cargo_binary_dir}/examples)
	add_subdirectory(examples)
endif()
