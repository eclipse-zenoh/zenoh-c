#
# Build script executed only if project is root project
# Contains tests, examples and installation procedure
#
set_default_build_type(Release)

declare_cache_var(ZENOHC_BUILD_EXAMPLES_WITH_STATIC_LIB FALSE BOOL "Use static zenohc lib for examples and tests")
declare_cache_var(ZENOHC_INSTALL_STATIC_LIBRARY FALSE BOOL "Install static library")

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
# Get filename property from library target,
# copy <name>.<ext> to <name>d.<ext> for install debug configuration
#
function(lib_property_to_install_file var target property)
	# avoid CMake behavior - setting variable to <VAR>-NOTFOUND for undefined property
    get_property(is_set TARGET ${target} PROPERTY ${property} SET)
    if (NOT is_set)
		unset(${var} PARENT_SCOPE)
		return()
	endif()
	get_property(oldname TARGET ${target} PROPERTY ${property})
	set(d $<$<CONFIG:Debug>:d>)
    get_filename_component(dir ${oldname} DIRECTORY)
    get_filename_component(name_we ${oldname} NAME_WE)
    get_filename_component(ext ${oldname} EXT)
    set(newname ${dir}/${name_we}${d}${ext})
	add_custom_command(TARGET cargo POST_BUILD
		COMMAND ${CMAKE_COMMAND} -E copy ${oldname} ${newname}
	)
	set(${var} ${newname} PARENT_SCOPE)
endfunction()

#
# Installation
# For debug configuration installs libraries with 'd' added to filename and
# package named 'zenohc_debug'
#
status_print(CMAKE_INSTALL_PREFIX)
lib_property_to_install_file(DYLIB zenohc::lib IMPORTED_LOCATION)
lib_property_to_install_file(IMPLIB zenohc::lib IMPORTED_IMPLIB)
lib_property_to_install_file(STATICLIB zenohc::static IMPORTED_LOCATION)
debug_print(DYLIB)
debug_print(IMPLIB)
debug_print(STATICLIB)

include(GNUInstallDirs)
set(CMAKE_INSTALL_INCLUDEDIR ${CMAKE_INSTALL_INCLUDEDIR}/zenohc)
install(FILES ${DYLIB} DESTINATION ${CMAKE_INSTALL_LIBDIR})
# if (${ZENOHC_INSTALL_STATIC_LIBRARY})
# 	install(FILES ${STATICLIB} DESTINATION ${CMAKE_INSTALL_LIBDIR})
# endif()
# get_target_property(include_dirs zenohc::lib INTERFACE_INCLUDE_DIRECTORIES)
# foreach(dir ${include_dirs})
# 	install(DIRECTORY "${dir}/" DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
# endforeach()
# if(APPLE OR UNIX)
# 	configure_file(${CMAKE_CURRENT_SOURCE_DIR}/zenohc.pc.in ${CMAKE_CURRENT_BINARY_DIR}/zenohc.pc @ONLY)
# 	install(FILES ${CMAKE_CURRENT_BINARY_DIR}/zenohc.pc DESTINATION "${CMAKE_INSTALL_LIBDIR}/pkgconfig" OPTIONAL)
# endif()

# set(CMAKE_INSTALL_CMAKEDIR "${CMAKE_INSTALL_LIBDIR}/cmake/${PROJECT_NAME}")

# # Generate <Package>Config.cmake
# include(CMakePackageConfigHelpers)
# configure_package_config_file(
#   "PackageConfig.cmake.in"
#   "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}Config.cmake"
#   INSTALL_DESTINATION "${CMAKE_INSTALL_CMAKEDIR}")

# # Generate <Package>Version.cmake
# write_basic_package_version_file(
#   "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}ConfigVersion.cmake"
#   VERSION ${PROJECT_VERSION}
#   COMPATIBILITY SameMajorVersion)

# install(
#   FILES "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}Config.cmake"
#         "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}ConfigVersion.cmake"
#   DESTINATION "${CMAKE_INSTALL_CMAKEDIR}"
#   COMPONENT dev)
