if(CPACK_GENERATOR MATCHES "DEB")
    # DEB package
<<<<<<< HEAD
||||||| parent of e85200e2 (fix: set debarch according to target not host)
    if(NOT DEBARCH)
        execute_process(
        COMMAND dpkg --print-architecture
        OUTPUT_VARIABLE DEBARCH
        OUTPUT_STRIP_TRAILING_WHITESPACE
        )
    endif()
=======
    if(NOT DEBARCH)
        set(DEBARCH ${CMAKE_SYSTEM_NAME}-${CMAKE_SYSTEM_PROCESSOR})
    endif()
>>>>>>> e85200e2 (fix: set debarch according to target not host)
    message(STATUS "Configure DEB packaging for Linux ${DEBARCH}")
    set(CPACK_DEBIAN_PACKAGE_MAINTAINER "ZettaScale Zenoh Team, <zenoh@zettascale.tech>")
    set(CPACK_DEB_COMPONENT_INSTALL ON)
    set(CPACK_DEBIAN_FILE_NAME DEB-DEFAULT)
    set(CPACK_DEBIAN_LIB_PACKAGE_NAME lib${CPACK_PACKAGE_NAME})
    set(CPACK_DEBIAN_LIB_PACKAGE_DEPENDS "libc6 (>=2.12)")
    set(CPACK_DEBIAN_DEV_PACKAGE_NAME lib${CPACK_PACKAGE_NAME}-dev)
    set(CPACK_DEBIAN_DEV_PACKAGE_DEPENDS "${CPACK_DEBIAN_LIB_PACKAGE_NAME} (=${CPACK_PACKAGE_VERSION})")
endif()

if(CPACK_GENERATOR MATCHES "RPM")
  # RPM package
  # rpmbuild should be installed
  # apt install rpm
  if(NOT RPMARCH)
    set(RPMARCH ${CMAKE_SYSTEM_PROCESSOR})
  endif()
  message(STATUS "Configure RPM packaging for Linux ${RPMARCH}")
  set(CPACK_RPM_PACKAGE_ARCHITECTURE ${RPMARCH})
  set(CPACK_RPM_COMPONENT_INSTALL ON)
  set(CPACK_RPM_FILE_NAME RPM-DEFAULT)
  set(CPACK_RPM_LIB_PACKAGE_NAME ${CPACK_PACKAGE_NAME}) # avoid "-lib" suffix for "lib" package
  set(CPACK_RPM_DEV_PACKAGE_REQUIRES "${CPACK_RPM_LIB_PACKAGE_NAME} = ${CPACK_PACKAGE_VERSION}")
endif()