cmake_minimum_required(VERSION 3.20)
project(
	zenohc
	VERSION 0.10.0.0
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
# Setup project version
#
set(project_version "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")
if(NOT PROJECT_VERSION_TWEAK)
	set(project_version "${project_version}-dev")
elseif(PROJECT_VERSION_TWEAK LESS 255)
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

set_genexpr_condition(cargo_toml_dir $<CONFIG:Debug> ${cargo_toml_dir_debug} ${cargo_toml_dir_release})
set_genexpr_condition(cargo_generated_include_dir $<CONFIG:Debug> ${cargo_toml_dir_debug}/include ${cargo_toml_dir_release}/include)

set(cargo_target_dir_debug ${cargo_toml_dir_debug}/target)
set(cargo_target_dir_release ${cargo_toml_dir_release}/target)
cmake_path(APPEND cargo_target_dir_debug ${ZENOHC_CUSTOM_TARGET})
cmake_path(APPEND cargo_target_dir_release ${ZENOHC_CUSTOM_TARGET})
set(cargo_binary_dir_debug ${cargo_target_dir_debug}/debug)
set(cargo_binary_dir_release ${cargo_target_dir_release}/release)
set(cargo_binary_dir $<IF:$<CONFIG:Debug>,${cargo_binary_dir_debug},${cargo_binary_dir_release}>)
set(source_include_dir ${CMAKE_CURRENT_SOURCE_DIR}/include)

function(configure_cargo_toml cargo_toml_dir CARGO_PROJECT_VERSION CARGO_LIB_NAME)
	message(STATUS "Configuring Cargo.toml in ${cargo_toml_dir} for ${CARGO_LIB_NAME}")
	if(NOT(cargo_toml_dir STREQUAL ${CMAKE_CURRENT_SOURCE_DIR}))
		set(CARGO_PROJECT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/")
		file(COPY
			${CMAKE_CURRENT_SOURCE_DIR}/splitguide.yaml
			${CMAKE_CURRENT_SOURCE_DIR}/cbindgen.toml
			${CMAKE_CURRENT_SOURCE_DIR}/rust-toolchain
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
	set(cargo_lib_name $<IF:$<CONFIG:Debug>,${cargo_lib_name_debug},${cargo_lib_name_release}>)
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

# dylib[d] - dymamic library (.so, .dll, .dylib)
# staticlib[d] - static library (.a, .lib)
# implib[d] - import library for windows dynamic library (DLL) - .lib
# dylibs[d] - list of files required for use dynamic libraty
# staticlibs[d] - list of files required for use static libraty
if(APPLE)
	set_lib(dylibsd dylibd "libzenohcd.dylib")
	set_lib(staticlibsd staticlibd "libzenohcd.a")
	set_lib(dylibs dylib "libzenohc.dylib")
	set_lib(staticlibs staticlib "libzenohc.a")
elseif(UNIX)
	set_lib(dylibs dylib "libzenohc.so")
	set_lib(staticlibs staticlib "libzenohc.a")
	set_lib(dylibsd dylibd "libzenohcd.so")
	set_lib(staticlibsd staticlibd "libzenohcd.a")
elseif(WIN32)
	set_lib(dylibs implib "zenohc.dll.lib")
	set_lib(dylibs dylib "zenohc.dll")
	set_lib(staticlibs staticlib "zenohc.lib")
	set_lib(dylibsd implibd "zenohcd.dll.lib")
	set_lib(dylibsd dylibd "zenohcd.dll")
	set_lib(staticlibsd staticlibd "zenohcd.lib")
endif()

list(APPEND libs ${dylibs})
list(APPEND libs ${staticlibs})
list(APPEND libsd ${dylibsd})
list(APPEND libsd ${staticlibsd})
list(TRANSFORM libs PREPEND "${cargo_binary_dir_release}/")
list(TRANSFORM libsd PREPEND "${cargo_binary_dir_debug}/")

#
# Build rust sources
#
set(cargo_flags ${ZENOHC_CARGO_FLAGS} $<$<NOT:$<CONFIG:Debug>>:--release>)
set(cargo_flags ${cargo_flags} --manifest-path=${cargo_toml_dir}/Cargo.toml)

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
	OUTPUT "$<IF:$<CONFIG:Debug>,${libsd},${libs}>"
	COMMAND ${CMAKE_COMMAND} -E echo \"RUSTFLAGS = $$RUSTFLAGS\"
	COMMAND ${CMAKE_COMMAND} -E echo \"cargo +${ZENOHC_CARGO_CHANNEL} build ${cargo_flags}\"
	COMMAND cargo +${ZENOHC_CARGO_CHANNEL} build ${cargo_flags}
)
add_custom_target(cargo ALL DEPENDS "$<IF:$<CONFIG:Debug>,${libsd},${libs}>")

#
# Define libraries built by cargo as targets
#
add_library(zenohc_static STATIC IMPORTED GLOBAL)
add_library(zenohc SHARED IMPORTED GLOBAL)
add_library(zenohc::static ALIAS zenohc_static)
add_library(zenohc::lib ALIAS zenohc)
add_dependencies(zenohc_static cargo)
add_dependencies(zenohc cargo)
get_required_static_libs(NATIVE_STATIC_LIBS)
target_link_libraries(zenohc_static INTERFACE ${NATIVE_STATIC_LIBS})

# Workaroud for https://github.com/rust-lang/cargo/issues/5045
# mentioned in https://github.com/eclipse-zenoh/zenoh-c/issues/138
# If it's fixed, do not forget to correct PackageConfig.cmake.in also
set_target_properties(zenohc PROPERTIES IMPORTED_NO_SONAME TRUE)

function(set_target_imported_locations target lib libd)
	set_target_properties(${target}
		PROPERTIES
		IMPORTED_GLOBAL TRUE
		IMPORTED_LOCATION_DEBUG ${cargo_binary_dir_debug}/${libd}
		IMPORTED_LOCATION_RELEASE ${cargo_binary_dir_release}/${lib}
		IMPORTED_LOCATION_MINSIZEREL ${cargo_binary_dir_release}/${lib}
		IMPORTED_LOCATION_RELWITHDEBINFO ${cargo_binary_dir_release}/${lib}
	)
endfunction()

function(set_target_imported_implib target lib libd)
	set_target_properties(${target}
		PROPERTIES
		IMPORTED_GLOBAL TRUE
		IMPORTED_IMPLIB_DEBUG ${cargo_binary_dir_debug}/${libd}
		IMPORTED_IMPLIB_RELEASE ${cargo_binary_dir_release}/${lib}
		IMPORTED_IMPLIB_MINSIZEREL ${cargo_binary_dir_release}/${lib}
		IMPORTED_IMPLIB_RELWITHDEBINFO ${cargo_binary_dir_release}/${lib}
	)
endfunction()

set_target_imported_locations(zenohc_static ${staticlib} ${staticlibd})
set_target_imported_locations(zenohc ${dylib} ${dylibd})

if(DEFINED implib)
	set_target_imported_implib(zenohc ${implib} ${implibd})
endif()

# Define include directories for library targets
status_print(source_include_dir)
status_print(cargo_generated_include_dir)
target_include_directories(zenohc_static INTERFACE ${source_include_dir})
target_include_directories(zenohc INTERFACE ${source_include_dir})

if(NOT(cargo_generated_include_dir STREQUAL ${source_include_dir}))
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

