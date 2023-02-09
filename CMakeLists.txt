cmake_minimum_required(VERSION 3.10)
project(
	zenohc
	VERSION 0.7.0.1
	DESCRIPTION "The C bindings for Zenoh"
	HOMEPAGE_URL "https://github.com/eclipse-zenoh/zenoh-c"
	LANGUAGES C
)

#
# Utility functions
#
function(status_print var)
	message(STATUS "${var} = ${${var}}")
endfunction()
function(declare_cache_var var default_value type docstring)
	set(${var} ${default_value} CACHE ${type} ${docstring})
	status_print(${var})
endfunction()

#
# Build configuration options
# 
# By default do build in source if current CMakeLists.txt is the root one
# and binary dir is "build" in it
set(in_ide FALSE)
if(CMAKE_CURRENT_BINARY_DIR STREQUAL "${CMAKE_CURRENT_SOURCE_DIR}/build")
	set(in_ide TRUE)
endif()
declare_cache_var(ZENOHC_BUILD_IN_SOURCE_TREE ${in_ide} BOOL "Do build inside source tree")
declare_cache_var(ZENOHC_BUILD_EXAMPLES_WITH_STATIC_LIB FALSE BOOL "Use static zenohc lib for examples and tests")
declare_cache_var(ZENOHC_BUILD_WITH_LOGGER_AUTOINIT TRUE BOOL "Enable logger-autoinit zenoh-c feature")
declare_cache_var(ZENOHC_INSTALL_STATIC_LIBRARY FALSE BOOL "Install static librayr")
declare_cache_var(ZENOHC_INSTALL_DEBUG FALSE BOOL "Protection from accidental installation of debug configuration")

#
# Generated expression are available only on build stage. Build 'debug' target to see values of
# variables printed with 'debug_print' function
#
add_custom_target(debug)
function(debug_print var)
	add_custom_command(
		COMMAND ${CMAKE_COMMAND} -E echo ${var} = ${${var}}
		TARGET debug
	)
endfunction()

#
# Select 'Release' as default config for single-config generator
# Remind user to select config type on build stage for multi-config generator
#
get_property(GENERATOR_IS_MULTI_CONFIG GLOBAL PROPERTY GENERATOR_IS_MULTI_CONFIG)
if(GENERATOR_IS_MULTI_CONFIG)
	message(STATUS "Multi-config generator selected, use cmake --build . --config [${CMAKE_CONFIGURATION_TYPES}]")
else()
	if(CMAKE_BUILD_TYPE STREQUAL "")
		set(CMAKE_BUILD_TYPE Release)
	endif()
	status_print(CMAKE_BUILD_TYPE)
endif()

#
# Prepare to build rust sources:
# configure Cargo.toml, copy files necessary for cargo, 
# create variables with path to cargo target directory
#
if(ZENOHC_BUILD_IN_SOURCE_TREE)
	set(cargo_toml_dir ${CMAKE_CURRENT_SOURCE_DIR})
	set(cargo_project_dir "") # do not put absoulte path into Cargo.toml if Cargo.toml is it's normal place
else()
	set(cargo_toml_dir ${CMAKE_CURRENT_BINARY_DIR})
	set(cargo_project_dir "${CMAKE_CURRENT_SOURCE_DIR}/")
	file(COPY 
		${cargo_project_dir}/splitguide.yaml 
		${cargo_project_dir}/cbindgen.toml
		${cargo_project_dir}/rust-toolchain 
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
status_print(dylibs)
status_print(staticlibs)
list(TRANSFORM dylibs PREPEND "${cargo_target_dir}/")
list(TRANSFORM staticlibs PREPEND "${cargo_target_dir}/")
debug_print(dylibs)
debug_print(staticlibs)

set(rustflags $ENV{RUSTFLAGS})
add_custom_target(cargo ALL
	COMMAND ${CMAKE_COMMAND} -E echo \"RUSTFLAGS = ${rustflags}\"
	COMMAND ${CMAKE_COMMAND} -E echo \"cargo build ${cargo_flags}\"
	COMMAND cargo build ${cargo_flags}
	BYPRODUCTS ${dylibs} ${staticlibs}
)
#
# Rust cross-build check for supported processor architectures
# This check works on linux only
# It requires that the following packages are installed for cross compilation:
#
# sudo apt install gcc-arm-linux-gnueabi
# sudo apt install gcc-x86-64-linux-gnu
# sudo apt install gcc-aarch64-linux-gnu
#
# and the following targets in rustup
#
# rustup target add arm-unknown-linux-gnueabi
# rustup target add aarch64-unknown-linux-gnu
# rustup target add x86_64-unknown-linux-gnu
# 
# check which targets are already installed with
# 
# rustup target list --installed
#
add_custom_target(crosscheck)

set(targets
aarch64-unknown-linux-gnu
x86_64-unknown-linux-gnu
arm-unknown-linux-gnueabi
)

foreach(target ${targets})
	add_custom_target(cargo_check_${target} 
		COMMAND cargo check --target ${target} ${cargo_flags} 
		COMMENT "cargo check on ${target}" 
		WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
	add_dependencies(crosscheck cargo_check_${target})
endforeach()

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

set_target_imported_locations(zenohc_static ${staticlib})
set_target_imported_locations(zenohc ${dylib})
if(DEFINED zenohc_implib)
	set_target_imported_implib(zenohc ${implib})
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

#
# Tests and examples
#
function(add_libraries target)
	if(ZENOHC_BUILD_EXAMPLES_WITH_STATIC_LIB)
		target_link_libraries(${target} PUBLIC zenohc::static)
	else()
		target_link_libraries(${target} PUBLIC zenohc::lib)
	endif()
	if(APPLE)
		find_library(FFoundation Foundation)
		find_library(FSecurity Security)
		target_link_libraries(${target} PUBLIC ${FFoundation} ${FSecurity})
	elseif(UNIX)
		target_link_libraries(${target} PUBLIC rt pthread m dl)
	elseif(WIN32)
		target_link_libraries(${target} PUBLIC ws2_32 crypt32 secur32 bcrypt ncrypt userenv ntdll iphlpapi runtimeobject)
	endif()

	if(WIN32)
		add_custom_command(TARGET ${target} POST_BUILD
			COMMAND ${CMAKE_COMMAND} -E copy_if_different $<TARGET_RUNTIME_DLLS:${target}> $<TARGET_FILE_DIR:${target}>
			COMMAND_EXPAND_LISTS
		)
	endif()
endfunction()

if(APPLE OR UNIX OR WIN32)
	file(GLOB examples_files "${CMAKE_CURRENT_SOURCE_DIR}/examples/*.c")
	add_custom_target(examples)

	foreach(file ${examples_files})
		get_filename_component(target ${file} NAME_WE)
		add_executable(${target} EXCLUDE_FROM_ALL ${file})
		add_dependencies(examples ${target})
		add_libraries(${target})
		set_property(TARGET ${target} PROPERTY C_STANDARD 11)
		set_property(TARGET ${target} PROPERTY RUNTIME_OUTPUT_DIRECTORY "${cargo_target_dir}/examples")
	endforeach()

	enable_testing()	
	file(GLOB tests_files "${CMAKE_CURRENT_SOURCE_DIR}/tests/*.c")
	add_custom_target(tests)

	foreach(file ${tests_files})
		get_filename_component(target ${file} NAME_WE)
		add_executable(${target} EXCLUDE_FROM_ALL ${file})
		add_dependencies(tests ${target})
		add_libraries(${target})
		set_property(TARGET ${target} PROPERTY C_STANDARD 11)
		set_property(TARGET ${target} PROPERTY RUNTIME_OUTPUT_DIRECTORY "${cargo_target_dir}/tests")
		add_test(NAME "test_${file}" COMMAND ${target})
	endforeach()
else()
	message(WARNING "You platform doesn't seem to support building the examples or tests.")
endif()

if(ZENOHC_INSTALL_DEBUG)
	set(configurations $<CONFIG>)
else()
	set(configurations "Release;RelWithDebInfo")
endif()
include(GNUInstallDirs)
set(CMAKE_INSTALL_INCLUDEDIR "${CMAKE_INSTALL_INCLUDEDIR}/zenohc")
install(FILES ${dylibs} CONFIGURATIONS ${configurations} DESTINATION ${CMAKE_INSTALL_LIBDIR})
if (${ZENOHC_INSTALL_STATIC_LIBRARY})
	install(FILES ${staticlibs} CONFIGURATIONS ${configurations} DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif()
install(DIRECTORY ${source_include_dir} CONFIGURATIONS ${configurations} DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
if(DEFINED ${cargo_generated_include_dir})
	install(DIRECTORY ${cargo_generated_include_dir} CONFIGURATIONS ${configurations} DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
endif()
if(APPLE OR UNIX)
	configure_file(${CMAKE_CURRENT_SOURCE_DIR}/zenohc.pc.in ${cargo_toml_dir}/zenohc.pc @ONLY)
	install(FILES ${cargo_toml_dir}/zenohc.pc CONFIGURATIONS ${configurations} DESTINATION "${CMAKE_INSTALL_LIBDIR}/pkgconfig" OPTIONAL)
endif()

set(CMAKE_INSTALL_CMAKEDIR "${CMAKE_INSTALL_LIBDIR}/cmake/${PROJECT_NAME}")

# Generate <Package>Config.cmake
include(CMakePackageConfigHelpers)
configure_package_config_file(
  "PackageConfig.cmake.in"
  "${PROJECT_NAME}Config.cmake"
  INSTALL_DESTINATION "${CMAKE_INSTALL_CMAKEDIR}")

# Generate <Package>Version.cmake
write_basic_package_version_file(
  "${PROJECT_NAME}ConfigVersion.cmake"
  VERSION ${PROJECT_VERSION}
  COMPATIBILITY SameMajorVersion)

install(
  FILES "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}Config.cmake"
        "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}ConfigVersion.cmake"
  DESTINATION "${CMAKE_INSTALL_CMAKEDIR}"
  COMPONENT dev)
