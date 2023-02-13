#
# Build zenohc library from rust sources
#
# Results of this script:
# target zenohc::lib - for linking zenoh-c as dynamic library
# target zenohc::static - for linking zenoh-c as static library
#

#
# Configuration options
#
declare_cache_var_true_if_vscode(ZENOHC_BUILD_IN_SOURCE_TREE "Do build inside source tree")
declare_cache_var(ZENOHC_BUILD_WITH_LOGGER_AUTOINIT TRUE BOOL "Enable logger-autoinit zenoh-c feature")

#
# Prepare to build rust sources:
# configure Cargo.toml, copy files necessary for cargo, 
# create variables with path to cargo target directory
#
# CARGO_PROJECT_DIR value used in Cargo.toml.in
# CMAKE_CURRENT_BINARY_DIR is set to rust
#
if(ZENOHC_BUILD_IN_SOURCE_TREE)
	set(cargo_toml_dir ${CMAKE_CURRENT_SOURCE_DIR})
    set(CMAKE_CURRENT_BINARY_DIR ${CMAKE_CURRENT_SOURCE_DIR}/target)
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

debug_print(source_include_dir)
debug_print(cargo_generated_include_dir)

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
status_print(dylibs)
status_print(staticlibs)
#
# add_custom_target(...BYPRODUCTS) do not support generator expressions in CMake < 3.20
# So pretend that cargo always builds both debug and release libraries
# instead of using generator expression which selects between debug/release
#
list(APPEND libs_debug ${dylibs})
list(APPEND libs_debug ${staticlibs})
list(APPEND libs_release ${dylibs})
list(APPEND libs_release ${staticlibs})
list(TRANSFORM libs_debug PREPEND "${CMAKE_CURRENT_BINARY_DIR}/debug/")
list(TRANSFORM libs_release PREPEND "${CMAKE_CURRENT_BINARY_DIR}/release/")

#
# Build rust sources
#
set(cargo_flags $<$<NOT:$<CONFIG:Debug>>:--release>)
if(ZENOHC_BUILD_WITH_LOGGER_AUTOINIT)
	set(cargo_flags ${cargo_flags} --features=logger-autoinit)
endif()
set(rustflags $ENV{RUSTFLAGS})
set(ENV{CARGO_TARGET_DIR} ${CMAKE_CURRENT_BINARY_DIR}) # result always in ${CMAKE_CURRENT_BINARY_DIR}/release or ../debug
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
add_library(zenohc::static ALIAS zenohc_static)
add_library(zenohc::lib ALIAS zenohc)
add_dependencies(zenohc_static cargo)
add_dependencies(zenohc cargo)

# Workaroud for https://github.com/rust-lang/cargo/issues/5045
# mentioned in https://github.com/eclipse-zenoh/zenoh-c/issues/138
set_target_properties(zenohc PROPERTIES IMPORTED_NO_SONAME TRUE)

function(set_target_imported_locations target libname)
	set_target_properties(${target}
		PROPERTIES 
		IMPORTED_GLOBAL TRUE
		IMPORTED_LOCATION $<IF:$<CONFIG:Debug>,${CMAKE_CURRENT_BINARY_DIR}/debug,${CMAKE_CURRENT_BINARY_DIR}/release>/${libname}
		IMPORTED_LOCATION_DEBUG ${CMAKE_CURRENT_BINARY_DIR}/debug/${libname}
		IMPORTED_LOCATION_RELEASE ${CMAKE_CURRENT_BINARY_DIR}/release/${libname}
		IMPORTED_LOCATION_MINSIZEREL ${CMAKE_CURRENT_BINARY_DIR}/release/${libname}
		IMPORTED_LOCATION_RELWITHDEBINFO ${CMAKE_CURRENT_BINARY_DIR}/release/${libname}
	)
endfunction()

function(set_target_imported_implib target libname)
	set_target_properties(${target}
		PROPERTIES 
		IMPORTED_GLOBAL TRUE
		IMPORTED_IMPLIB $<IF:$<CONFIG:Debug>,${CMAKE_CURRENT_BINARY_DIR}/debug,${CMAKE_CURRENT_BINARY_DIR}/release>/${libname}
		IMPORTED_IMPLIB_DEBUG ${CMAKE_CURRENT_BINARY_DIR}/debug/${libname}
		IMPORTED_IMPLIB_RELEASE ${CMAKE_CURRENT_BINARY_DIR}/release/${libname}
		IMPORTED_IMPLIB_MINSIZEREL ${CMAKE_CURRENT_BINARY_DIR}/release/${libname}
		IMPORTED_IMPLIB_RELWITHDEBINFO ${CMAKE_CURRENT_BINARY_DIR}/release/${libname}
	)
endfunction()

set_target_imported_locations(zenohc_static ${staticlib})
set_target_imported_locations(zenohc ${dylib})
if(DEFINED zenohc_implib)
	set_target_imported_implib(zenohc ${implib})
endif()

# Define include directories for library targets
target_include_directories(zenohc_static INTERFACE ${source_include_dir})
target_include_directories(zenohc INTERFACE ${source_include_dir})
if(DEFINED cargo_generated_include_dir)
	file(MAKE_DIRECTORY ${cargo_generated_include_dir})
	target_include_directories(zenohc_static INTERFACE ${cargo_generated_include_dir})
	target_include_directories(zenohc INTERFACE ${cargo_generated_include_dir})
endif()

set_target_properties(zenohc zenohc_static PROPERTIES IMPORTED_GLOBAL TRUE)

