cmake_minimum_required(VERSION 3.10)
project(
	zenohc
	VERSION 0.6.0.1
	DESCRIPTION "The C bindings for Zenoh"
	HOMEPAGE_URL "https://github.com/eclipse-zenoh/zenoh-c"
	LANGUAGES C
)
set(CARGO_PROJECT_VERSION "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")

if(NOT PROJECT_VERSION_TWEAK)
	set(CARGO_PROJECT_VERSION "${CARGO_PROJECT_VERSION}-dev")
elseif(PROJECT_VERSION_TWEAK LESS 255)
	set(CARGO_PROJECT_VERSION "${CARGO_PROJECT_VERSION}-beta.${PROJECT_VERSION_TWEAK}")
endif()

message(STATUS "CARGO_PROJECT_VERSION: ${CARGO_PROJECT_VERSION}")
configure_file("${CMAKE_SOURCE_DIR}/Cargo.toml.in" "${CMAKE_SOURCE_DIR}/Cargo.toml" @ONLY)

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

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
	set(CMAKE_BINARY_DIR "${CMAKE_SOURCE_DIR}/target/debug")
elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
	set(CARGO_FLAGS ${CARGO_FLAGS} "--release")
	set(CMAKE_BINARY_DIR "${CMAKE_SOURCE_DIR}/target/release")
elseif(CMAKE_BUILD_TYPE STREQUAL "RelWithDebInfo")
	set(CARGO_FLAGS ${CARGO_FLAGS} "--release")
	set(ENV{RUSTFLAGS} "-g")
	set(CMAKE_BINARY_DIR "${CMAKE_SOURCE_DIR}/target/release")
elseif(CMAKE_BUILD_TYPE STREQUAL "MinSizeRel")
	set(CARGO_FLAGS ${CARGO_FLAGS} "--release")
	set(CMAKE_BINARY_DIR "${CMAKE_SOURCE_DIR}/target/release")
else()
	message(FATAL_ERROR "Unknown CMAKE_BUILD_TYPE: ${CMAKE_BUILD_TYPE}")
endif()

message(STATUS "CMAKE_BINARY_DIR: ${CMAKE_BINARY_DIR}")

if(NOT DISABLE_LOGGER_AUTOINIT)
	set(CARGO_FLAGS ${CARGO_FLAGS} "--features=logger-autoinit")
endif()

set(CARGO_FLAGS ${CARGO_FLAGS} "--manifest-path=${CMAKE_SOURCE_DIR}/Cargo.toml")
set(RUSTFLAGS $ENV{RUSTFLAGS})
add_custom_target(cargo ALL COMMAND cargo build ${CARGO_FLAGS} COMMENT "Running RUSTFLAGS=${RUSTFLAGS} cargo build ${CARGO_FLAGS}")

if(APPLE)
	set(libzenohc "${CMAKE_BINARY_DIR}/libzenohc.dylib")
	set(libzenohc_static "${CMAKE_BINARY_DIR}/libzenohc.a")
elseif(UNIX)
	set(libzenohc "${CMAKE_BINARY_DIR}/libzenohc.so")
	set(libzenohc_static "${CMAKE_BINARY_DIR}/libzenohc.a")
elseif(WIN32)
	set(libzenohc "${CMAKE_BINARY_DIR}/zenohc.dll")
	set(libzenohc_static "${CMAKE_BINARY_DIR}/zenohc.lib")
endif()

add_library(zenohc_static STATIC IMPORTED)
add_dependencies(zenohc_static cargo)
message(STATUS "zenoh-c static library will be built at: ${libzenohc_static}")
set_property(TARGET zenohc_static PROPERTY IMPORTED_LOCATION ${libzenohc_static})

add_library(zenohc SHARED IMPORTED)
add_dependencies(zenohc cargo)
message(STATUS "zenoh-c dynamic library will be built at: ${libzenohc}")
set_property(TARGET zenohc PROPERTY IMPORTED_LOCATION ${libzenohc})
target_include_directories(zenohc INTERFACE "${CMAKE_SOURCE_DIR}/include")

if(APPLE OR UNIX OR WIN32)
	file(GLOB examples_files "${CMAKE_SOURCE_DIR}/examples/*.c")
	add_custom_target(examples)

	foreach(file ${examples_files})
		get_filename_component(target ${file} NAME_WE)
		add_executable(${target} EXCLUDE_FROM_ALL ${file})
		add_dependencies(examples ${target})
		add_dependencies(${target} zenohc_static)
		set_property(TARGET ${target} PROPERTY C_STANDARD 11)
		set_property(TARGET ${target} PROPERTY RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/examples")
		target_include_directories(${target} PUBLIC "${CMAKE_SOURCE_DIR}/include")
		target_link_libraries(${target} PUBLIC ${libzenohc_static})

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

	file(GLOB tests_files "${CMAKE_SOURCE_DIR}/tests/*.c")
	add_custom_target(tests)

	foreach(file ${tests_files})
		get_filename_component(target ${file} NAME_WE)
		add_executable(${target} EXCLUDE_FROM_ALL ${file})
		add_dependencies(tests ${target})
		add_dependencies(${target} zenohc_static)
		set_property(TARGET ${target} PROPERTY C_STANDARD 11)
		set_property(TARGET ${target} PROPERTY RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/tests")
		target_include_directories(${target} PUBLIC "${CMAKE_SOURCE_DIR}/include")
		target_link_libraries(${target} PUBLIC ${libzenohc_static})

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
	message(STATUS "Protect from accidental installing debug build into system to avoid unexpected performance degradation")
	set(allowed_to_install "Release;RelWithDebInfo" CACHE INTERNAL "Configurations allowed to install")
else()
	set(allowed_to_install "Debug;Release;MinSizeRel;RelWithDebInfo" CACHE INTERNAL "Configurations allowed to install")
endif()
message(STATUS "CMAKE_INSTALL_PREFIX: ${CMAKE_INSTALL_PREFIX}")
message(STATUS "Configurations allowed to install: ${allowed_to_install}")
include(GNUInstallDirs)
set(CMAKE_INSTALL_INCLUDEDIR "${CMAKE_INSTALL_INCLUDEDIR}/zenohc")
install(FILES ${libzenohc} CONFIGURATIONS ${allowed_to_install} DESTINATION ${CMAKE_INSTALL_LIBDIR})
install(DIRECTORY "${CMAKE_SOURCE_DIR}/include/" CONFIGURATIONS ${allowed_to_install} DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
configure_file("${CMAKE_SOURCE_DIR}/zenohc.pc.in" "${CMAKE_SOURCE_DIR}/zenohc.pc" @ONLY)

if(APPLE OR UNIX)
	install(FILES "${CMAKE_SOURCE_DIR}/zenohc.pc" CONFIGURATIONS ${allowed_to_install} DESTINATION "${CMAKE_INSTALL_LIBDIR}/pkgconfig" OPTIONAL)
endif()