message(STATUS "zenoh-c install")

declare_cache_var(ZENOHC_INSTALL_STATIC_LIBRARY FALSE BOOL "Install zenoh-c static library")

add_custom_target(zenohc_prepare_files_to_install ALL)

#
# adds 'd' to library filename if installing the debug configuration
#
function(get_library_file var target property)
	# avoid CMake behavior - setting variable to <VAR>-NOTFOUND for undefined property
    get_property(is_set TARGET ${target} PROPERTY ${property} SET)
    if (NOT is_set)
		unset(${var} PARENT_SCOPE)
		return()
	endif()
	get_property(oldname TARGET ${target} PROPERTY ${property})
    add_dependencies(zenohc_prepare_files_to_install ${target})
	set(d $<$<CONFIG:Debug>:d>)
    get_filename_component(dir ${oldname} DIRECTORY)
    get_filename_component(name_we ${oldname} NAME_WE)
    get_filename_component(ext ${oldname} EXT)
    set(newname ${CMAKE_CURRENT_BINARY_DIR}/${name_we}${d}${ext})
	add_custom_command(TARGET zenohc_prepare_files_to_install POST_BUILD
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
get_library_file(DYLIB zenohc::lib IMPORTED_LOCATION)
get_library_file(IMPLIB zenohc::lib IMPORTED_IMPLIB)
get_library_file(STATICLIB zenohc::static IMPORTED_LOCATION)

include(GNUInstallDirs)
set(CMAKE_INSTALL_INCLUDEDIR ${CMAKE_INSTALL_INCLUDEDIR}/zenohc)
install(FILES ${DYLIB} ${IMPLIB} DESTINATION ${CMAKE_INSTALL_LIBDIR})
if(ZENOHC_INSTALL_STATIC_LIBRARY)
    install(FILES ${STATICLIB} DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif()
get_target_property(include_dirs zenohc::lib INTERFACE_INCLUDE_DIRECTORIES)
foreach(dir ${include_dirs})
	install(DIRECTORY "${dir}/" DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
endforeach()
if(APPLE OR UNIX)
	configure_file(${CMAKE_CURRENT_SOURCE_DIR}/zenohc.pc.in ${CMAKE_CURRENT_BINARY_DIR}/zenohc.pc @ONLY)
	install(FILES ${CMAKE_CURRENT_BINARY_DIR}/zenohc.pc DESTINATION "${CMAKE_INSTALL_LIBDIR}/pkgconfig" OPTIONAL)
endif()

set(CMAKE_INSTALL_CMAKEDIR "${CMAKE_INSTALL_LIBDIR}/cmake/${PROJECT_NAME}")

# Generate <Package>Config.cmake
include(CMakePackageConfigHelpers)
configure_package_config_file(
  "PackageConfig.cmake.in"
  "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}Config.cmake"
  INSTALL_DESTINATION "${CMAKE_INSTALL_CMAKEDIR}")

# Generate <Package>Version.cmake
write_basic_package_version_file(
  "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}ConfigVersion.cmake"
  VERSION ${PROJECT_VERSION}
  COMPATIBILITY SameMajorVersion)

install(
  FILES "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}Config.cmake"
        "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}ConfigVersion.cmake"
  DESTINATION "${CMAKE_INSTALL_CMAKEDIR}"
  COMPONENT dev)
