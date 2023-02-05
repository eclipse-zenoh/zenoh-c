cmake_minimum_required(VERSION 3.10)
project(
	zenohc
	VERSION 0.7.0.1
	DESCRIPTION "The C bindings for Zenoh"
	HOMEPAGE_URL "https://github.com/eclipse-zenoh/zenoh-c"
	LANGUAGES C
)
set_property(GLOBAL PROPERTY GLOBAL_DEPENDS_DEBUG_MODE 1)

#
# Prepare to build rust sources:
# configure Cargo.toml, copy files necessary for cargo, 
# create variables with path to cargo target directory
#

# When work in IDE it's convenient to build rust sources into "target" 
# in source tree and place generated headers. Otherwise it's better to keep
# source tree untouched.
# By default do build in source if current CMakeLists.txt is the root one
# and binary dir is "build" in it
set(in_ide FALSE)
if(CMAKE_CURRENT_BINARY_DIR STREQUAL "${CMAKE_CURRENT_SOURCE_DIR}/build")
	set(in_ide TRUE)
endif()
option(BUILD_IN_SOURCE_TREE "Do build inside source tree" ${in_ide})
message(STATUS "Building inside source tree: ${BUILD_IN_SOURCE_TREE}")

if(BUILD_IN_SOURCE_TREE)
	set(CARGO_TOML_DIR ${CMAKE_CURRENT_SOURCE_DIR})
	set(CARGO_SOURCE_DIR "") # do not put absoulte path into Cargo.toml if Cargo.toml is it's normal place
else()
	set(CARGO_TOML_DIR ${CMAKE_CURRENT_BINARY_DIR})
	set(CARGO_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/")
	file(COPY 
		${CARGO_SOURCE_DIR}/splitguide.yaml 
		${CARGO_SOURCE_DIR}/cbindgen.toml
		${CARGO_SOURCE_DIR}/rust-toolchain 
		DESTINATION ${CARGO_TOML_DIR})
endif()
set(is_debug $<CONFIG:Debug>)
set(is_release $<NOT:$<CONFIG:Debug>>)
set(CARGO_TARGET_DIR_DEBUG ${CARGO_TOML_DIR}/target/debug)
set(CARGO_TARGET_DIR_RELEASE ${CARGO_TOML_DIR}/target/release)
set(CARGO_TARGET_DIR $<IF:${is_debug},${CARGO_TARGET_DIR_DEBUG},${CARGO_TARGET_DIR_RELEASE}>)
set(CARGO_GENERATED_INCLUDE_DIR ${CARGO_TOML_DIR}/include)

set(CARGO_PROJECT_VERSION "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")

if(NOT PROJECT_VERSION_TWEAK)
	set(CARGO_PROJECT_VERSION "${CARGO_PROJECT_VERSION}-dev")
elseif(PROJECT_VERSION_TWEAK LESS 255)
	set(CARGO_PROJECT_VERSION "${CARGO_PROJECT_VERSION}-rc.${PROJECT_VERSION_TWEAK}")
endif()

message(STATUS "CARGO_PROJECT_VERSION: ${CARGO_PROJECT_VERSION}")
configure_file("${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml.in" "${CARGO_TOML_DIR}/Cargo.toml" @ONLY)

set(CARGO_FLAGS $<${is_release}:--release>)
option(DISABLE_LOGGER_AUTOINIT "Disable logger-autoinit feature" FALSE)
if(NOT DISABLE_LOGGER_AUTOINIT)
	set(CARGO_FLAGS ${CARGO_FLAGS} --features=logger-autoinit)
endif()

#
# Build rust sources
#
if(APPLE)
	set(zenohc_dylib "libzenohc.dylib")
	set(zenohc_staticlib "libzenohc.a")
elseif(UNIX)
	set(zenohc_dylib "libzenohc.so")
	set(zenohc_staticlib "libzenohc.a")
elseif(WIN32)
	set(zenohc_implib "zenohc.dll.lib")
	set(zenohc_dylib "zenohc.dll")
	set(zenohc_staticlib "zenohc.lib")
endif()


set(RUSTFLAGS $ENV{RUSTFLAGS})
add_custom_target(cargo ALL
	COMMAND ${CMAKE_COMMAND} -E echo \"RUSTFLAGS = ${RUSTFLAGS}\"
	COMMAND ${CMAKE_COMMAND} -E echo \"cargo build ${CARGO_FLAGS}\"
	COMMAND cargo build ${CARGO_FLAGS}
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
		COMMAND cargo check --target ${target} ${CARGO_FLAGS} 
		COMMENT "cargo check on ${target}" 
		WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
	add_dependencies(crosscheck cargo_check_${target})
endforeach()

#
# Import libraries built by cargo as targets
#

function(set_target_imported_locations target libname)
	set_target_properties(${target}
		PROPERTIES 
		IMPORTED_GLOBAL TRUE
		IMPORTED_LOCATION_DEBUG ${CARGO_TARGET_DIR_DEBUG}/${libname}
		IMPORTED_LOCATION_RELEASE ${CARGO_TARGET_DIR_RELEASE}/${libname}
		IMPORTED_LOCATION_MINSIZEREL ${CARGO_TARGET_DIR_RELEASE}/${libname}
		IMPORTED_LOCATION_RELWITHDEBINFO ${CARGO_TARGET_DIR_RELEASE}/${libname}
	)
endfunction()

function(set_target_imported_implib target libname)
	set_target_properties(${target}
		PROPERTIES 
		IMPORTED_GLOBAL TRUE
		IMPORTED_IMPLIB_DEBUG ${CARGO_TARGET_DIR_DEBUG}/${libname}
		IMPORTED_IMPLIB_RELEASE ${CARGO_TARGET_DIR_RELEASE}/${libname}
		IMPORTED_IMPLIB_MINSIZEREL ${CARGO_TARGET_DIR_RELEASE}/${libname}
		IMPORTED_IMPLIB_RELWITHDEBINFO ${CARGO_TARGET_DIR_RELEASE}/${libname}
	)
endfunction()

add_library(zenohc_static STATIC IMPORTED)
add_dependencies(zenohc_static cargo)
target_include_directories(zenohc_static 
	INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/include 
	INTERFACE ${CARGO_GENERATED_INCLUDE_DIR}
)
set_target_imported_locations(zenohc_static ${zenohc_staticlib})

add_library(zenohc SHARED IMPORTED)
add_dependencies(zenohc cargo)
target_include_directories(zenohc 
	INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/include 
	INTERFACE ${CARGO_GENERATED_INCLUDE_DIR}
)
set_target_imported_locations(zenohc ${zenohc_dylib})
if(DEFINED zenohc_implib)
	set_target_imported_implib(zenohc ${zenohc_implib})
endif()

set_target_properties(zenohc zenohc_static PROPERTIES IMPORTED_GLOBAL TRUE)
add_library(zenohc::static ALIAS zenohc_static)
add_library(zenohc::lib ALIAS zenohc)

#
# Generated expression are available only on build stage. Build this target to see their actual values
#
add_custom_target(debug)
function(debug_print var)
	add_custom_command(
		COMMAND ${CMAKE_COMMAND} -E echo ${var} = ${${var}}
		TARGET debug
	)
endfunction()
debug_print(CARGO_TARGET_DIR)
debug_print(CARGO_GENERATED_INCLUDE_DIR)
debug_print(zenohc_implib)
debug_print(zenohc_dylib)
debug_print(zenohc_staticlib)

#
# Tests and examples
#
option(USE_STATIC_LIB "Use static zenohc lib for examples and tests" FALSE)
message(STATUS "Build examples and tests with static lib : USE_STATIC_LIB = ${USE_STATIC_LIB}")
function(add_libraries target)
	if(USE_STATIC_LIB)
		add_dependencies(${target} zenohc::static )
		target_link_libraries(${target} PUBLIC zenohc::static)
	else()
		add_dependencies(${target} zenohc::lib )
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
		set_property(TARGET ${target} PROPERTY RUNTIME_OUTPUT_DIRECTORY "${CARGO_TARGET_DIR}/examples")
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
		set_property(TARGET ${target} PROPERTY RUNTIME_OUTPUT_DIRECTORY "${CARGO_TARGET_DIR}/tests")
		add_test(NAME "test_${file}" COMMAND ${target})
	endforeach()
else()
	message(WARNING "You platform doesn't seem to support building the examples or tests.")
endif()

# if(CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
# 	set(ZENOHC_INSTALL_CONFIGURATIONS "Release;RelWithDebInfo" CACHE STRING "Configurations allowed to install")
# 	set(ZENOHC_INSTALL_STATIC_LIBRARY FALSE CACHE BOOL "Install static library")
# else()
# 	set(ZENOHC_INSTALL_CONFIGURATIONS "Debug;Release;MinSizeRel;RelWithDebInfo" CACHE INTERNAL "Configurations allowed to install")
# 	set(ZENOHC_INSTALL_STATIC_LIBRARY TRUE CACHE BOOL "Install static library")
# endif()
# message(STATUS "CMAKE_INSTALL_PREFIX: ${CMAKE_INSTALL_PREFIX}")
# message(STATUS "Configurations allowed to install: ${ZENOHC_INSTALL_CONFIGURATIONS}")
# message(STATUS "Static library will be installed: ${ZENOHC_INSTALL_STATIC_LIBRARY}")
# include(GNUInstallDirs)
# set(CMAKE_INSTALL_INCLUDEDIR "${CMAKE_INSTALL_INCLUDEDIR}/zenohc")
# install(FILES ${CMAKE_BINARY_DIR}/${libzenohc} CONFIGURATIONS ${ZENOHC_INSTALL_CONFIGURATIONS} DESTINATION ${CMAKE_INSTALL_LIBDIR})
# if (${ZENOHC_INSTALL_STATIC_LIBRARY})
# 	install(FILES ${CMAKE_BINARY_DIR}/${libzenohc_static} CONFIGURATIONS ${ZENOHC_INSTALL_CONFIGURATIONS} DESTINATION ${CMAKE_INSTALL_LIBDIR})
# endif()
# install(DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/include/" CONFIGURATIONS ${ZENOHC_INSTALL_CONFIGURATIONS} DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
# configure_file("${CMAKE_CURRENT_SOURCE_DIR}/zenohc.pc.in" "${CMAKE_CURRENT_SOURCE_DIR}/zenohc.pc" @ONLY)

# if(APPLE OR UNIX)
# 	install(FILES "${CMAKE_CURRENT_SOURCE_DIR}/zenohc.pc" CONFIGURATIONS ${ZENOHC_INSTALL_CONFIGURATIONS} DESTINATION "${CMAKE_INSTALL_LIBDIR}/pkgconfig" OPTIONAL)
# endif()

# set(CMAKE_INSTALL_CMAKEDIR "${CMAKE_INSTALL_LIBDIR}/cmake/${PROJECT_NAME}")

# # Generate <Package>Config.cmake
# include(CMakePackageConfigHelpers)
# configure_package_config_file(
#   "PackageConfig.cmake.in"
#   "${PROJECT_NAME}Config.cmake"
#   INSTALL_DESTINATION "${CMAKE_INSTALL_CMAKEDIR}")

# # Generate <Package>Version.cmake
# write_basic_package_version_file(
#   "${PROJECT_NAME}ConfigVersion.cmake"
#   VERSION ${PROJECT_VERSION}
#   COMPATIBILITY SameMajorVersion)

# install(
#   FILES "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}Config.cmake"
#         "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}ConfigVersion.cmake"
#   DESTINATION "${CMAKE_INSTALL_CMAKEDIR}"
#   COMPONENT dev)
