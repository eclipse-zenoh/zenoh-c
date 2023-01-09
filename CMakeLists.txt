cmake_minimum_required(VERSION 3.10)
project(
	zenohc
	VERSION 0.7.0.1
	DESCRIPTION "The C bindings for Zenoh"
	HOMEPAGE_URL "https://github.com/eclipse-zenoh/zenoh-c"
	LANGUAGES C
)
set(CARGO_PROJECT_VERSION "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")

if(NOT PROJECT_VERSION_TWEAK)
	set(CARGO_PROJECT_VERSION "${CARGO_PROJECT_VERSION}-dev")
elseif(PROJECT_VERSION_TWEAK LESS 255)
	set(CARGO_PROJECT_VERSION "${CARGO_PROJECT_VERSION}-rc.${PROJECT_VERSION_TWEAK}")
endif()

message(STATUS "CARGO_PROJECT_VERSION: ${CARGO_PROJECT_VERSION}")
configure_file("${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml.in" "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml" @ONLY)

if(NOT CMAKE_BUILD_TYPE)
	set(default_build_type "RelWithDebInfo")
	message(STATUS "Setting build type to '${default_build_type}' as none was specified.")
	set(CMAKE_BUILD_TYPE "${default_build_type}" CACHE
		STRING "Choose the type of build." FORCE)

	# Set the possible values of build type for cmake-gui
	set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS
		"Debug" "Release" "MinSizeRel" "RelWithDebInfo")
endif()

set(CARGO_FLAGS "")

# allow to set cross-compilation cargo target
set(CARGO_TARGET "" CACHE STRING "Set target triple for cargo or leave empty for default")
if(${CARGO_TARGET})
	set(CARGO_FLAGS ${CARGO_FLAGS} "--target ${CARGO_TARGET}")
endif()

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
	set(CMAKE_BINARY_DIR "${CMAKE_CURRENT_SOURCE_DIR}/target/debug")
elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
	set(CARGO_FLAGS ${CARGO_FLAGS} "--release")
	set(CMAKE_BINARY_DIR "${CMAKE_CURRENT_SOURCE_DIR}/target/release")
elseif(CMAKE_BUILD_TYPE STREQUAL "RelWithDebInfo")
	set(CARGO_FLAGS ${CARGO_FLAGS} "--release")
	set(ENV{RUSTFLAGS} "-g")
	set(CMAKE_BINARY_DIR "${CMAKE_CURRENT_SOURCE_DIR}/target/release")
elseif(CMAKE_BUILD_TYPE STREQUAL "MinSizeRel")
	set(CARGO_FLAGS ${CARGO_FLAGS} "--release")
	set(CMAKE_BINARY_DIR "${CMAKE_CURRENT_SOURCE_DIR}/target/release")
else()
	message(FATAL_ERROR "Unknown CMAKE_BUILD_TYPE: ${CMAKE_BUILD_TYPE}")
endif()

message(STATUS "CMAKE_BINARY_DIR: ${CMAKE_BINARY_DIR}")

if(NOT DISABLE_LOGGER_AUTOINIT)
	set(CARGO_FLAGS ${CARGO_FLAGS} "--features=logger-autoinit")
endif()

set(CARGO_FLAGS ${CARGO_FLAGS} "--manifest-path=${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml")
set(RUSTFLAGS $ENV{RUSTFLAGS})
add_custom_target(cargo ALL COMMAND cargo build ${CARGO_FLAGS} COMMENT "Running RUSTFLAGS=${RUSTFLAGS} cargo build ${CARGO_FLAGS}" WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}")

#
# Rust cross-build check for supported processor architectures
#
add_custom_target(check_proc_arch)

set(supported_targets
aarch64-apple-darwin
aarch64-unknown-linux-gnu
x86_64-apple-darwin
x86_64-unknown-linux-gnu
x86_64-pc-windows-msvc
arm-unknown-linux-gnueabi
arm-unknown-linux-gnueabihf
)

foreach(target ${supported_targets})
	add_custom_target(target_rustup_add_${target} COMMAND rustup target add ${target} "rustup add ${target}")
	add_custom_target(target_cargo_check_${target} COMMAND cargo check --target ${target} ${CARGO_FLAGS} "cargo check on ${target}" WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}")
	add_dependencies(targets target_cargo_check_${target})
	add_dependencies(target_cargo_check_${target} target_rustup_add_${target})
endforeach()

if(APPLE)
	set(libzenohc "libzenohc.dylib")
	set(libzenohc_static "libzenohc.a")
elseif(UNIX)
	set(libzenohc "libzenohc.so")
	set(libzenohc_static "libzenohc.a")
elseif(WIN32)
	set(libzenohc "zenohc.dll")
	set(libzenohc_static "zenohc.lib")
endif()

add_library(zenohc_static STATIC IMPORTED)
set_target_properties(zenohc_static PROPERTIES IMPORTED_GLOBAL TRUE)
add_library(zenohc::static ALIAS zenohc_static)
add_dependencies(zenohc_static cargo)
message(STATUS "zenoh-c static library will be built at: ${CMAKE_BINARY_DIR}/${libzenohc_static}")
set_property(TARGET zenohc_static PROPERTY IMPORTED_LOCATION "${CMAKE_BINARY_DIR}/${libzenohc_static}")

add_library(zenohc SHARED IMPORTED)
set_target_properties(zenohc PROPERTIES IMPORTED_GLOBAL TRUE)
add_library(zenohc::lib ALIAS zenohc)
add_dependencies(zenohc cargo)
message(STATUS "zenoh-c dynamic library will be built at: ${CMAKE_BINARY_DIR}/${libzenohc}")
set_property(TARGET zenohc PROPERTY IMPORTED_LOCATION "${CMAKE_BINARY_DIR}/${libzenohc}")
target_include_directories(zenohc INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}/include")

if(APPLE OR UNIX OR WIN32)
	file(GLOB examples_files "${CMAKE_CURRENT_SOURCE_DIR}/examples/*.c")
	add_custom_target(examples)

	foreach(file ${examples_files})
		get_filename_component(target ${file} NAME_WE)
		add_executable(${target} EXCLUDE_FROM_ALL ${file})
		add_dependencies(examples ${target})
		add_dependencies(${target} zenohc_static)
		set_property(TARGET ${target} PROPERTY C_STANDARD 11)
		set_property(TARGET ${target} PROPERTY RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/examples")
		target_include_directories(${target} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
		target_link_libraries(${target} PUBLIC "${CMAKE_BINARY_DIR}/${libzenohc_static}")

		if(APPLE)
			find_library(FFoundation Foundation)
			find_library(FSecurity Security)
			target_link_libraries(${target} PUBLIC ${FFoundation} ${FSecurity})
		elseif(UNIX)
			target_link_libraries(${target} PUBLIC rt pthread m dl)
		elseif(WIN32)
			target_link_libraries(${target} PUBLIC ws2_32 crypt32 secur32 bcrypt ncrypt userenv ntdll iphlpapi runtimeobject)
		endif()
	endforeach()

	file(GLOB tests_files "${CMAKE_CURRENT_SOURCE_DIR}/tests/*.c")
	add_custom_target(tests)

	foreach(file ${tests_files})
		get_filename_component(target ${file} NAME_WE)
		add_executable(${target} EXCLUDE_FROM_ALL ${file})
		add_dependencies(tests ${target})
		add_dependencies(${target} zenohc_static)
		set_property(TARGET ${target} PROPERTY C_STANDARD 11)
		set_property(TARGET ${target} PROPERTY RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/tests")
		target_include_directories(${target} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
		target_link_libraries(${target} PUBLIC "${CMAKE_BINARY_DIR}/${libzenohc_static}")

		if(APPLE)
			find_library(FFoundation Foundation)
			find_library(FSecurity Security)
			target_link_libraries(${target} PUBLIC ${FFoundation} ${FSecurity})
		elseif(UNIX)
			target_link_libraries(${target} PUBLIC rt pthread m dl)
		elseif(WIN32)
			target_link_libraries(${target} PUBLIC ws2_32 crypt32 secur32 bcrypt ncrypt userenv ntdll iphlpapi runtimeobject)
		endif()
	endforeach()
else()
	message(WARNING "You platform doesn't seem to support building the examples or tests.")
endif()

if(CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
	set(ZENOHC_INSTALL_CONFIGURATIONS "Release;RelWithDebInfo" CACHE STRING "Configurations allowed to install")
	set(ZENOHC_INSTALL_STATIC_LIBRARY FALSE CACHE BOOL "Install static library")
else()
	set(ZENOHC_INSTALL_CONFIGURATIONS "Debug;Release;MinSizeRel;RelWithDebInfo" CACHE INTERNAL "Configurations allowed to install")
	set(ZENOHC_INSTALL_STATIC_LIBRARY TRUE CACHE BOOL "Install static library")
endif()
message(STATUS "CMAKE_INSTALL_PREFIX: ${CMAKE_INSTALL_PREFIX}")
message(STATUS "Configurations allowed to install: ${ZENOHC_INSTALL_CONFIGURATIONS}")
message(STATUS "Static library will be installed: ${ZENOHC_INSTALL_STATIC_LIBRARY}")
include(GNUInstallDirs)
set(CMAKE_INSTALL_INCLUDEDIR "${CMAKE_INSTALL_INCLUDEDIR}/zenohc")
install(FILES ${CMAKE_BINARY_DIR}/${libzenohc} CONFIGURATIONS ${ZENOHC_INSTALL_CONFIGURATIONS} DESTINATION ${CMAKE_INSTALL_LIBDIR})
if (${ZENOHC_INSTALL_STATIC_LIBRARY})
	install(FILES ${CMAKE_BINARY_DIR}/${libzenohc_static} CONFIGURATIONS ${ZENOHC_INSTALL_CONFIGURATIONS} DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif()
install(DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/include/" CONFIGURATIONS ${ZENOHC_INSTALL_CONFIGURATIONS} DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
configure_file("${CMAKE_CURRENT_SOURCE_DIR}/zenohc.pc.in" "${CMAKE_CURRENT_SOURCE_DIR}/zenohc.pc" @ONLY)

if(APPLE OR UNIX)
	install(FILES "${CMAKE_CURRENT_SOURCE_DIR}/zenohc.pc" CONFIGURATIONS ${ZENOHC_INSTALL_CONFIGURATIONS} DESTINATION "${CMAKE_INSTALL_LIBDIR}/pkgconfig" OPTIONAL)
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
