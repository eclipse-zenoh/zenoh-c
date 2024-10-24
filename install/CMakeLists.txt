message(STATUS "zenoh-c install")

#
# Installation
# For debug configuration installs libraries with 'd' added to filename
#
status_print(CMAKE_INSTALL_PREFIX)

include(GNUInstallDirs)
include(CMakePackageConfigHelpers)

function(install_zenohc_lib configurations property_postfix package_name)
    get_target_property(dylib_path zenohc::shared IMPORTED_LOCATION_${property_postfix})
    get_target_property_if_set(implib_path zenohc::shared IMPORTED_IMPLIB_${property_postfix})
    get_filename_component(DYLIB ${dylib_path} NAME)
    get_filename_component(IMPLIB "${implib_path}" NAME)
    # On Windows .dll need to be installed in ${CMAKE_INSTALL_BINDIR}, 
    # while on Linux and macOS .so and .dylib need to be installed in ${CMAKE_INSTALL_LIBDIR}
    if(WIN32)
        set(ZENOHC_INSTALL_DYLIBDIR ${CMAKE_INSTALL_BINDIR})
    else()
        set(ZENOHC_INSTALL_DYLIBDIR ${CMAKE_INSTALL_LIBDIR})
    endif()
    install(FILES ${dylib_path} DESTINATION ${ZENOHC_INSTALL_DYLIBDIR} CONFIGURATIONS ${configurations} COMPONENT lib)
    if(DEFINED implib_path)
        install(FILES ${implib_path} DESTINATION ${CMAKE_INSTALL_LIBDIR} CONFIGURATIONS ${configurations} COMPONENT lib)
    endif()
    if((APPLE OR UNIX))
        get_filename_component(LIBNAME ${DYLIB} NAME_WE)
        configure_file(${CMAKE_CURRENT_SOURCE_DIR}/zenohc.pc.in ${CMAKE_CURRENT_BINARY_DIR}/${package_name}_${property_postfix}.pc @ONLY)
        install(FILES ${CMAKE_CURRENT_BINARY_DIR}/${package_name}_${property_postfix}.pc 
            DESTINATION "${CMAKE_INSTALL_LIBDIR}/pkgconfig" 
            CONFIGURATIONS ${configurations}
            RENAME ${package_name}.pc
            OPTIONAL
            COMPONENT dev)
    endif()

    get_target_property(staticlib_path zenohc::static IMPORTED_LOCATION_${property_postfix})
    get_target_property(NATIVE_STATIC_LIBS zenohc::static INTERFACE_LINK_LIBRARIES)
    get_filename_component(STATICLIB ${staticlib_path} NAME)
    install(FILES ${staticlib_path} DESTINATION ${CMAKE_INSTALL_LIBDIR} CONFIGURATIONS ${configurations} COMPONENT lib)

    set(CMAKE_INSTALL_CMAKEDIR "${CMAKE_INSTALL_LIBDIR}/cmake/${package_name}")

    # Generate <Package>Config.cmake
    configure_package_config_file(
        "PackageConfig.cmake.in"
        "${CMAKE_CURRENT_BINARY_DIR}/${package_name}_${property_postfix}Config.cmake"
        INSTALL_DESTINATION "${CMAKE_INSTALL_CMAKEDIR}")

    # Generate <Package>Version.cmake
    write_basic_package_version_file(
        "${CMAKE_CURRENT_BINARY_DIR}/${package_name}_${property_postfix}ConfigVersion.cmake"
        VERSION ${PROJECT_VERSION}
        COMPATIBILITY SameMajorVersion)

    install(
        FILES "${CMAKE_CURRENT_BINARY_DIR}/${package_name}_${property_postfix}Config.cmake"
        DESTINATION "${CMAKE_INSTALL_CMAKEDIR}"
        CONFIGURATIONS ${configurations}
        RENAME ${package_name}Config.cmake
        COMPONENT dev)

    install(
        FILES "${CMAKE_CURRENT_BINARY_DIR}/${package_name}_${property_postfix}ConfigVersion.cmake"
        DESTINATION "${CMAKE_INSTALL_CMAKEDIR}"
        CONFIGURATIONS ${configurations}
        RENAME ${package_name}ConfigVersion.cmake
        COMPONENT dev)


endfunction()


#
# Install headers
#
get_target_property(include_dirs zenohc::lib INTERFACE_INCLUDE_DIRECTORIES)
foreach(dir ${include_dirs})
    install(DIRECTORY "${dir}/" DESTINATION ${CMAKE_INSTALL_INCLUDEDIR} COMPONENT headers)
endforeach()

#
# Install lib files 
#
install_zenohc_lib("Release;RelWithDebInfo;MinSizeRel;None" "RELEASE" zenohc)
install_zenohc_lib("Debug" "DEBUG" zenohc)

#
# Configure CPack
#
set(CPACK_BUILD_CONFIG ${CMAKE_BUILD_TYPE})
set(CPACK_PACKAGE_DIRECTORY "${CMAKE_BINARY_DIR}/packages")
set(CPACK_PACKAGE_CHECKSUM MD5)
set(CPACK_PACKAGE_VENDOR "The Eclipse Foundation")
set(CPACK_PACKAGE_VERSION_MAJOR ${PROJECT_VERSION_MAJOR})
set(CPACK_PACKAGE_VERSION_MINOR ${PROJECT_VERSION_MINOR})
set(CPACK_PACKAGE_VERSION_PATCH ${PROJECT_VERSION_PATCH})
set(CPACK_COMPONENT_LIB_DESCRIPTION "The C client library for Eclipse zenoh")
set(CPACK_COMPONENT_DEV_DESCRIPTION "${CPACK_COMPONENT_LIB_DESCRIPTION} - cmake files")
set(CPACK_COMPONENT_HEADERS_DESCRIPTION "${CPACK_COMPONENT_LIB_DESCRIPTION} - header files")
set(CPACK_COMPONENT_LIB_GROUP "lib")
set(CPACK_COMPONENT_HEADERS_GROUP "dev")
set(CPACK_COMPONENT_DEV_GROUP "dev")
set(CPACK_COMPONENT_HEADERS_DEPENDS lib)
set(CPACK_COMPONENT_DEV_DEPENDS lib)

set(CPACK_GENERATOR ZIP)
# set package name to format <name>-<version>.<extension>
# platform name doesn't matter for header-only library
set(CPACK_PACKAGE_NAME zenohc)

if(NOT CPACK_PACKAGE_VERSION)
    set(SEM_VER "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")
    if(PROJECT_VERSION_TWEAK STREQUAL "")
        set(CPACK_PACKAGE_VERSION ${SEM_VER})
    elseif(PROJECT_VERSION_TWEAK EQUAL 0)
        set(CPACK_PACKAGE_VERSION "${SEM_VER}~dev-1")
    elseif(PROJECT_VERSION_TWEAK GREATER 0)
        set(CPACK_PACKAGE_VERSION "${SEM_VER}~pre.${PROJECT_VERSION_TWEAK}-1")
    endif()
endif()

set(CPACK_PACKAGE_FILE_NAME "${CPACK_PACKAGE_NAME}-${CPACK_PACKAGE_VERSION}-${CMAKE_SYSTEM_NAME}-${CMAKE_SYSTEM_PROCESSOR}")

set(CPACK_DEBIAN_PACKAGE_ARCHITECTURE ${CMAKE_SYSTEM_NAME}-${CMAKE_SYSTEM_PROCESSOR})
set(CPACK_PROJECT_CONFIG_FILE "${CMAKE_CURRENT_SOURCE_DIR}/cpack_project_config.cmake")

include(CPack)
