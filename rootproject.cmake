set_default_build_type(Release)

declare_cache_var(ZENOHC_BUILD_EXAMPLES_WITH_STATIC_LIB FALSE BOOL "Use static zenohc lib for examples and tests")
declare_cache_var(ZENOHC_INSTALL_STATIC_LIBRARY FALSE BOOL "Install static librayr")
declare_cache_var(ZENOHC_INSTALL_DEBUG FALSE BOOL "Protection from accidental installation of debug configuration")

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

#
# Installation
#
status_print(CMAKE_INSTALL_PREFIX)
list(TRANSFORM DYLIBS PREPEND "${cargo_target_dir}/")
list(TRANSFORM STATICLIBS PREPEND "${cargo_target_dir}/")
debug_print(DYLIBS)
debug_print(STATICLIBS)
if(ZENOHC_INSTALL_DEBUG)
	set(configurations $<CONFIG>)
else()
	set(configurations "Release;RelWithDebInfo")
endif()
include(GNUInstallDirs)
set(CMAKE_INSTALL_INCLUDEDIR "${CMAKE_INSTALL_INCLUDEDIR}/zenohc")
install(FILES ${DYLIBS} CONFIGURATIONS ${configurations} DESTINATION ${CMAKE_INSTALL_LIBDIR})
if (${ZENOHC_INSTALL_STATIC_LIBRARY})
	install(FILES ${STATICLIBS} CONFIGURATIONS ${configurations} DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif()
install(DIRECTORY "${source_include_dir}/" CONFIGURATIONS ${configurations} DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
if(DEFINED ${cargo_generated_include_dir})
	install(DIRECTORY "${cargo_generated_include_dir}/" CONFIGURATIONS ${configurations} DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
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
