cmake_minimum_required(VERSION 3.16)

# NOTE(fuzzpixelz): The ci/scripts/bump.bash scripts sets the project version
# using regular expressions on this statement.
set(ZENOHC_VERSION 0.11.0.0)

project(
	zenohc
	VERSION ${ZENOHC_VERSION}
	DESCRIPTION "The C bindings for Zenoh"
	HOMEPAGE_URL "https://github.com/eclipse-zenoh/zenoh-c"
	LANGUAGES C
)

set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake" ${CMAKE_MODULE_PATH})
include(helpers)
set_default_build_type(Release)
enable_testing()

#
# Build zenohc library from rust sources
#
# target zenohc::lib - for linking zenoh-c as static or dynamic library depending on ZENOHC_STATIC boolean cache var
# target zenohc::static - for linking zenoh-c as static library
# target zenohc::shared - for linking zenoh-c as dynamic library
#

#
# Configuration options
#
declare_cache_var_true_if_vscode(ZENOHC_BUILD_IN_SOURCE_TREE "Do build inside source tree")
declare_cache_var(ZENOHC_BUILD_WITH_LOGGER_AUTOINIT TRUE BOOL "Enable logger-autoinit zenoh-c feature")
declare_cache_var(ZENOHC_BUILD_WITH_SHARED_MEMORY TRUE BOOL "Enable shared-memory zenoh-c feature")
declare_cache_var(ZENOHC_CUSTOM_TARGET "" STRING "Rust target for cross compilation, 'aarch64-unknown-linux-gnu' for example")
declare_cache_var(ZENOHC_CARGO_CHANNEL "" STRING "Cargo channel parameter. Should be '+stable', '+nightly' or empty value")
declare_cache_var(ZENOHC_CARGO_FLAGS "" STRING "Additional cargo flags")
declare_cache_var(ZENOHC_LIB_STATIC FALSE BOOL "Alias zenohc::lib target to zenohc::static if TRUE, to zenohc::shared if FALSE")

#
# Setup project version
#
set(project_version "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")
if(NOT PROJECT_VERSION_TWEAK)
	set(project_version "${project_version}")
elseif(PROJECT_VERSION_TWEAK EQUAL 0)
	set(project_version "${project_version}-dev")
elseif(PROJECT_VERSION_TWEAK GREATER 1)
	set(project_version "${project_version}-rc.${PROJECT_VERSION_TWEAK}")
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

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
	set(DEBUG TRUE)
endif()

set_genexpr_condition(cargo_toml_dir DEBUG $<CONFIG:Debug> ${cargo_toml_dir_debug} ${cargo_toml_dir_release})
set_genexpr_condition(cargo_generated_include_dir DEBUG $<CONFIG:Debug> ${cargo_toml_dir_debug}/include ${cargo_toml_dir_release}/include)

set(cargo_target_dir_debug ${cargo_toml_dir_debug}/target)
set(cargo_target_dir_release ${cargo_toml_dir_release}/target)
if(NOT(ZENOHC_CUSTOM_TARGET STREQUAL ""))
	set(cargo_target_dir_debug ${cargo_target_dir_debug}/${ZENOHC_CUSTOM_TARGET})
	set(cargo_target_dir_release ${cargo_target_dir_release}/${ZENOHC_CUSTOM_TARGET})
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
			${CMAKE_CURRENT_SOURCE_DIR}/Cargo.lock
			${CMAKE_CURRENT_SOURCE_DIR}/rust-toolchain.toml
			DESTINATION ${cargo_toml_dir})
	endif()	
	configure_file("${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml.in" "${cargo_toml_dir}/Cargo.toml" @ONLY)
endfunction()

#
# Configure Cargo.toml files
#
set(cargo_lib_name_debug zenohcd)
set(cargo_lib_name_release zenohc)
if(cargo_toml_dir_debug STREQUAL cargo_toml_dir_release)
	# same Cargo.toml is for ide, debug and release configurations
	# This happens only for non-multiconfig generators, so testing for debug/release on configuration stage is allowed
	if (CMAKE_BUILD_TYPE STREQUAL "Debug")
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
# Configure result library names
#
macro(set_lib list var value)
	set(${var} ${value})
	list(APPEND ${list} ${value})
endmacro()

# dylib[r|d] - dymamic library (.so, .dll, .dylib)
# staticlib[r|d] - static library (.a, .lib)
# implib[r|d] - import library for windows dynamic library (DLL) - .lib
# dylibs[r|d] - list of files required for use dynamic libraty
# staticlibs[r|d] - list of files required for use static libraty
set_lib(dylibsr dylibr ${CMAKE_SHARED_LIBRARY_PREFIX}zenohc${CMAKE_SHARED_LIBRARY_SUFFIX})
set_lib(dylibsd dylibd ${CMAKE_SHARED_LIBRARY_PREFIX}zenohcd${CMAKE_SHARED_LIBRARY_SUFFIX})
set_lib(staticlibsr staticlibr ${CMAKE_STATIC_LIBRARY_PREFIX}zenohc${CMAKE_STATIC_LIBRARY_SUFFIX})
set_lib(staticlibsd staticlibd ${CMAKE_STATIC_LIBRARY_PREFIX}zenohcd${CMAKE_STATIC_LIBRARY_SUFFIX})
if(WIN32)
	set_lib(dylibsr implibr ${CMAKE_IMPORT_LIBRARY_PREFIX}zenohc${CMAKE_SHARED_LIBRARY_SUFFIX}${CMAKE_IMPORT_LIBRARY_SUFFIX})
	set_lib(dylibsd implibd ${CMAKE_IMPORT_LIBRARY_PREFIX}zenohcd${CMAKE_SHARED_LIBRARY_SUFFIX}${CMAKE_IMPORT_LIBRARY_SUFFIX})
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
status_print(libs)
file(GLOB_RECURSE rust_sources "Cargo.toml.in" "src/*.rs" "build.rs" "splitguide.yaml")
add_custom_command(
	OUTPUT ${libs}
	COMMAND ${CMAKE_COMMAND} -E echo \"RUSTFLAGS = $$RUSTFLAGS\"
	COMMAND ${CMAKE_COMMAND} -E echo \"cargo ${ZENOHC_CARGO_CHANNEL} build ${cargo_flags}\"
	COMMAND cargo ${ZENOHC_CARGO_CHANNEL} build ${cargo_flags}
	VERBATIM
	COMMAND_EXPAND_LISTS
	DEPENDS "${rust_sources}"
)
add_custom_target(cargo ALL DEPENDS "${libs}")

#
# Define libraries built by cargo as targets
#
add_library(zenohc_static STATIC IMPORTED GLOBAL)
add_library(zenohc_shared SHARED IMPORTED GLOBAL)
add_library(zenohc::static ALIAS zenohc_static)
add_library(zenohc::shared ALIAS zenohc_shared)
if (ZENOHC_LIB_STATIC)
	add_library(zenohc::lib ALIAS zenohc_static)
else()
	add_library(zenohc::lib ALIAS zenohc_shared)
endif()
add_dependencies(zenohc_static cargo)
add_dependencies(zenohc_shared cargo)

#
# Setup additional properties for library targets
# *IMPORTANT* any options in this section should be repeated in install/PackageConfig.cmake.in
# to achieve correct behavior of find_package(zenohc)
#
get_required_static_libs(NATIVE_STATIC_LIBS)
target_link_libraries(zenohc_static INTERFACE ${NATIVE_STATIC_LIBS})
target_compile_definitions(zenohc_shared INTERFACE ZENOHC_DYN_LIB)

# Workaroud for https://github.com/rust-lang/cargo/issues/5045
# mentioned in https://github.com/eclipse-zenoh/zenoh-c/issues/138
# If it's fixed, do not forget to correct PackageConfig.cmake.in also
set_target_properties(zenohc_shared PROPERTIES IMPORTED_NO_SONAME TRUE)

function(set_target_imported_locations target libr libd)
	set_target_properties(${target}
		PROPERTIES
		IMPORTED_GLOBAL TRUE
		IMPORTED_LOCATION_DEBUG ${cargo_binary_dir_debug}/${libd}
		IMPORTED_LOCATION_RELEASE ${cargo_binary_dir_release}/${libr}
		IMPORTED_LOCATION_MINSIZEREL ${cargo_binary_dir_release}/${libr}
		IMPORTED_LOCATION_RELWITHDEBINFO ${cargo_binary_dir_release}/${libr}
	)
endfunction()

function(set_target_imported_implib target libr libd)
	set_target_properties(${target}
		PROPERTIES
		IMPORTED_GLOBAL TRUE
		IMPORTED_IMPLIB_DEBUG ${cargo_binary_dir_debug}/${libd}
		IMPORTED_IMPLIB_RELEASE ${cargo_binary_dir_release}/${libr}
		IMPORTED_IMPLIB_MINSIZEREL ${cargo_binary_dir_release}/${libr}
		IMPORTED_IMPLIB_RELWITHDEBINFO ${cargo_binary_dir_release}/${libr}
	)
endfunction()

set_target_imported_locations(zenohc_static ${staticlibr} ${staticlibd})
set_target_imported_locations(zenohc_shared ${dylibr} ${dylibd})

if(DEFINED implibr)
	set_target_imported_implib(zenohc_shared ${implibr} ${implibd})
endif()

# Define include directories for library targets
status_print(source_include_dir)
status_print(cargo_generated_include_dir)
target_include_directories(zenohc_static INTERFACE ${source_include_dir})
target_include_directories(zenohc_shared INTERFACE ${source_include_dir})

if(NOT(cargo_generated_include_dir STREQUAL ${source_include_dir}))
	target_include_directories(zenohc_static INTERFACE ${cargo_generated_include_dir})
	target_include_directories(zenohc_shared INTERFACE ${cargo_generated_include_dir})
endif()

set_target_properties(zenohc_shared zenohc_static PROPERTIES IMPORTED_GLOBAL TRUE)

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

